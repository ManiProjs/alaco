mod analyzer;
mod ast;
mod codegen;
mod compiler;
mod error;
mod features;
mod ir;
mod ir_validate;
mod language;
mod lexer;
mod lsp;
mod manifest;
mod optimizer;
mod parser;
mod stdlib;
mod token;

use std::{
    fs,
    io::IsTerminal,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{Duration, Instant},
};

use clap::{Parser, Subcommand};
use miette::Result;
use owo_colors::OwoColorize;

use compiler::Compiler;

#[derive(Parser, Debug)]
#[command(
    name = "alaco",
    version,
    about = "The Alaco programming language toolchain",
    long_about = "The everything tool for Alaco: create projects, check code, build and run programs, execute test programs, clean build artifacts, and diagnose your development environment.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<CommandKind>,
}

#[derive(Subcommand, Debug)]
enum CommandKind {
    /// Create a new Alaco project.
    New {
        /// Name of the project and its new directory.
        name: String,
    },

    /// Initialize an Alaco project in an existing directory.
    Init {
        /// Directory to initialize (defaults to the current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Check an Alaco source file.
    Check { file: Option<PathBuf> },

    /// Transpile an Alaco source file to JavaScript.
    Transpile {
        file: Option<PathBuf>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Explain an error in an Alaco source file.
    Explain { file: Option<PathBuf> },

    /// Build an Alaco source file.
    Build {
        file: Option<PathBuf>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compile and run an Alaco program.
    Run { file: Option<PathBuf> },

    /// Compile and run test programs from tests/ (or a supplied file/directory).
    Test {
        /// A .aco test file or directory containing .aco test files (defaults to tests/).
        #[arg(default_value = "tests")]
        path: PathBuf,
    },

    /// Remove generated project build artifacts from target/.
    Clean,

    /// Check whether the Alaco build tools are installed correctly.
    Doctor,

    /// Validate and display the current package manifest.
    Manifest,

    /// Add a library from a Git repository and record it in Alaco.toml.
    Add {
        /// Git repository URL (HTTPS or SSH).
        git_repo: String,
    },

    /// Start the Alaco Language Server Protocol (LSP) server over stdio.
    Lsp,
}

fn main() -> ExitCode {
    let use_color = should_use_color(
        std::io::stdout().is_terminal(),
        std::io::stderr().is_terminal(),
        std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()),
    );
    let _ = miette::set_hook(Box::new(move |_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .color(use_color)
                .unicode(true)
                .terminal_links(true)
                .build(),
        )
    }));

    match run_cli() {
        Ok(code) => code,

        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}

fn should_use_color(stdout_is_terminal: bool, stderr_is_terminal: bool, no_color: bool) -> bool {
    stdout_is_terminal && stderr_is_terminal && !no_color
}

fn colors_enabled() -> bool {
    should_use_color(
        std::io::stdout().is_terminal(),
        std::io::stderr().is_terminal(),
        std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()),
    )
}

fn green(value: impl std::fmt::Display) -> String {
    let text = value.to_string();
    if colors_enabled() {
        text.green().to_string()
    } else {
        text
    }
}

fn red(value: impl std::fmt::Display) -> String {
    let text = value.to_string();
    if colors_enabled() {
        text.red().to_string()
    } else {
        text
    }
}

fn bold(value: impl std::fmt::Display) -> String {
    let text = value.to_string();
    if colors_enabled() {
        text.bold().to_string()
    } else {
        text
    }
}

fn dimmed(value: impl std::fmt::Display) -> String {
    let text = value.to_string();
    if colors_enabled() {
        text.dimmed().to_string()
    } else {
        text
    }
}

fn run_cli() -> Result<ExitCode> {
    let cli = Cli::parse();

    match cli.command {
        Some(CommandKind::New { name }) => new_project(&name),

        Some(CommandKind::Init { path }) => init_project(&path),

        Some(CommandKind::Check { file }) => check(&resolve_source(file)?),

        Some(CommandKind::Transpile { file, output }) => {
            transpile(&resolve_source(file)?, output.as_deref())
        }

        Some(CommandKind::Explain { file }) => explain(&resolve_source(file)?),

        Some(CommandKind::Build { file, output }) => {
            build(&resolve_source(file)?, output.as_deref())
        }

        Some(CommandKind::Run { file }) => run(&resolve_source(file)?),

        Some(CommandKind::Test { path }) => test(&path),

        Some(CommandKind::Clean) => clean(),

        Some(CommandKind::Doctor) => doctor(),

        Some(CommandKind::Manifest) => show_manifest(),

        Some(CommandKind::Add { git_repo }) => add_dependency(&git_repo),

        Some(CommandKind::Lsp) => lsp::run()
            .map(|()| ExitCode::SUCCESS)
            .map_err(|error| miette::miette!("LSP transport error: {error}")),

        None => unreachable!("Clap requires a subcommand"),
    }
}

fn resolve_source(file: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(file) = file {
        return Ok(file);
    }

    let entry = if Path::new("Alaco.toml").is_file() {
        crate::manifest::Manifest::load(Path::new("Alaco.toml"))?.entry
    } else {
        PathBuf::from("src/main.aco")
    };
    if entry.is_file() {
        return Ok(entry);
    }

    Err(miette::miette!(
        "no source file was provided and the package entry '{}' was not found; check Alaco.toml or pass a source file",
        entry.display()
    ))
}

fn new_project(name: &str) -> Result<ExitCode> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\\')
        || name.chars().any(char::is_control)
    {
        return Err(miette::miette!(
            "invalid project name '{name}'; use a simple directory name"
        ));
    }

    let root = PathBuf::from(name);
    if root.exists() {
        return Err(miette::miette!(
            "cannot create project '{}': the path already exists",
            root.display()
        ));
    }

    create_project_files(&root)?;
    println!("  {} Created project `{name}`", green("✓"));
    println!("\n  Next steps:\n    cd {name}\n    alaco run");

    Ok(ExitCode::SUCCESS)
}

fn init_project(root: &Path) -> Result<ExitCode> {
    if !root.is_dir() {
        return Err(miette::miette!(
            "project directory '{}' does not exist",
            root.display()
        ));
    }

    let entry = root.join("src/main.aco");
    if entry.exists() {
        return Err(miette::miette!(
            "'{}' already exists; refusing to overwrite it",
            entry.display()
        ));
    }

    create_project_files(root)?;
    println!(
        "  {} Initialized Alaco project in {}",
        green("✓"),
        root.display()
    );

    Ok(ExitCode::SUCCESS)
}

fn create_project_files(root: &Path) -> Result<()> {
    let source_dir = root.join("src");
    fs::create_dir_all(&source_dir)
        .map_err(|error| miette::miette!("failed to create '{}': {error}", source_dir.display()))?;

    let project_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("alaco-project");
    let package_name = project_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let manifest = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2026\"\nentry = \"src/main.aco\"\n\n[dependencies]\n",
        package_name
    );
    write_if_missing(&root.join("Alaco.toml"), &manifest)?;

    fs::write(
        source_dir.join("main.aco"),
        "fn main() {\n    println(\"Hello, Alaco!\");\n}\n",
    )
    .map_err(|error| miette::miette!("failed to write project entry point: {error}"))?;

    write_if_missing(&root.join(".gitignore"), "/target/\n/.alaco/\n")?;
    write_if_missing(
        &root.join("README.md"),
        "# Alaco project\n\nRun the program with `alaco run`.\n\nCheck it with `alaco check`, or build it with `alaco build`.\n",
    )?;

    Ok(())
}

fn write_if_missing(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    fs::write(path, contents)
        .map_err(|error| miette::miette!("failed to write '{}': {error}", path.display()))
}

fn doctor() -> Result<ExitCode> {
    let mut failed = false;
    println!("Alaco {} environment check\n", env!("CARGO_PKG_VERSION"));

    match Command::new("clang++").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let first_line = version.lines().next().unwrap_or("clang++ found");
            println!("  {} C++ compiler: {first_line}", green("✓"));
        }
        Ok(_) => {
            failed = true;
            println!("  {} clang++ is installed but failed to run", red("✗"));
        }
        Err(error) => {
            failed = true;
            println!("  {} clang++ not found ({error})", red("✗"));
            println!("    Install LLVM/Clang and ensure `clang++` is on PATH.");
        }
    }

    let stdlib = crate::stdlib::Stdlib::new();
    let stdlib_root = stdlib.root();
    if stdlib_root.is_dir() {
        println!(
            "  {} Standard library: {}",
            green("✓"),
            stdlib_root.display()
        );
    } else {
        failed = true;
        println!(
            "  {} Standard library not found: {}",
            red("✗"),
            stdlib_root.display()
        );
    }

    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

fn show_manifest() -> Result<ExitCode> {
    let path = PathBuf::from("Alaco.toml");
    let manifest = crate::manifest::Manifest::load(&path)?;
    println!("{} {}", bold(&manifest.name), manifest.version);
    println!("  Edition: {}", manifest.edition);
    println!("  Entry:   {}", manifest.entry.display());
    println!("  Target:  {}", manifest.build.target);
    println!("  Backend: {}", manifest.build.backend);
    println!("  Compiler: {}", manifest.build.compiler);
    println!("  Cache:   {}", manifest.build.cache);
    println!("  Source map: {}", manifest.build.source_map);
    if !manifest.build.flags.is_empty() {
        println!("  Flags:   {}", manifest.build.flags.join(" "));
    }
    println!("  Dependencies: {}", manifest.dependencies.len());
    for (name, dependency) in &manifest.dependencies {
        println!("    {name} = {}", dependency.display_description());
    }
    Ok(ExitCode::SUCCESS)
}

fn add_dependency(git_repo: &str) -> Result<ExitCode> {
    if git_repo.trim().is_empty() || git_repo.chars().any(char::is_control) {
        return Err(miette::miette!("provide a valid Git repository URL"));
    }

    let manifest_path = PathBuf::from("Alaco.toml");
    let manifest = crate::manifest::Manifest::load(&manifest_path)?;
    let root = std::env::current_dir()
        .map_err(|error| miette::miette!("cannot determine current directory: {error}"))?;
    let package_dir = root.join(".alaco").join("packages");
    fs::create_dir_all(&package_dir).map_err(|error| {
        miette::miette!(
            "failed to create package directory '{}': {error}",
            package_dir.display()
        )
    })?;

    let temporary = package_dir.join(format!(".alaco-add-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary).map_err(|error| {
            miette::miette!("failed to clear temporary package directory: {error}")
        })?;
    }

    let clone = Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg("--")
        .arg(git_repo)
        .arg(&temporary)
        .status()
        .map_err(|error| miette::miette!("failed to run Git; install Git and retry: {error}"))?;
    if !clone.success() {
        let _ = fs::remove_dir_all(&temporary);
        return Err(miette::miette!("Git clone failed for '{git_repo}'"));
    }

    let package_manifest_path = temporary.join("Alaco.toml");
    let package_manifest = match crate::manifest::Manifest::load(&package_manifest_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            let _ = fs::remove_dir_all(&temporary);
            return Err(miette::miette!(
                "repository '{git_repo}' is not an Alaco package: {error}"
            ));
        }
    };
    let dependency_name = package_manifest.name.replace('-', "_");
    if dependency_name == "std"
        || !dependency_name
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        || !dependency_name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        let _ = fs::remove_dir_all(&temporary);
        return Err(miette::miette!(
            "package name '{}' cannot be used as an Alaco import name",
            package_manifest.name
        ));
    }
    if manifest
        .dependencies
        .keys()
        .any(|name| name.replace('-', "_") == dependency_name)
    {
        let _ = fs::remove_dir_all(&temporary);
        return Err(miette::miette!(
            "dependency '{dependency_name}' is already declared in Alaco.toml"
        ));
    }

    let revision_output = Command::new("git")
        .arg("-C")
        .arg(&temporary)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| miette::miette!("failed to read cloned Git revision: {error}"))?;
    if !revision_output.status.success() {
        let _ = fs::remove_dir_all(&temporary);
        return Err(miette::miette!(
            "could not determine the Git revision for '{git_repo}'"
        ));
    }
    let revision = String::from_utf8_lossy(&revision_output.stdout)
        .trim()
        .to_owned();

    if let Err(error) =
        install_git_dependency_tree(&temporary, &mut vec![format!("git:{git_repo}")])
    {
        let _ = fs::remove_dir_all(&temporary);
        return Err(error);
    }

    let destination = package_dir.join(&dependency_name);
    if destination.exists() {
        let _ = fs::remove_dir_all(&temporary);
        return Err(miette::miette!(
            "package directory '{}' already exists; remove it or choose a different dependency name",
            destination.display()
        ));
    }

    let source = fs::read_to_string(&manifest_path)
        .map_err(|error| miette::miette!("cannot read '{}': {error}", manifest_path.display()))?;
    let git_value = toml::Value::String(git_repo.to_owned());
    let rev_value = toml::Value::String(revision.clone());
    let declaration = format!("{dependency_name} = {{ git = {git_value}, rev = {rev_value} }}");
    let updated = insert_dependency(&source, &declaration);

    fs::rename(&temporary, &destination).map_err(|error| {
        miette::miette!(
            "failed to install package into '{}': {error}",
            destination.display()
        )
    })?;
    if let Err(error) = fs::write(&manifest_path, updated) {
        let _ = fs::remove_dir_all(&destination);
        return Err(miette::miette!(
            "failed to update '{}': {error}",
            manifest_path.display()
        ));
    }

    println!("  {} Added library `{dependency_name}`", green("✓"));
    println!("    Repository: {git_repo}");
    println!("    Revision:   {revision}");
    println!("    Installed:  {}", destination.display());
    println!(
        "\n  Import it in your source with `import {dependency_name};` or `import {dependency_name}.module;`."
    );
    Ok(ExitCode::SUCCESS)
}

/// Install every Git and local-path dependency of a package, recursively.
/// Git dependencies are checked out at their declared revision (when present)
/// and the parent manifest is pinned to the resolved commit SHA.
fn install_git_dependency_tree(package_root: &Path, active: &mut Vec<String>) -> Result<()> {
    if active.len() > 128 {
        return Err(miette::miette!(
            "package dependency graph exceeds 128 nested packages; check for a dependency cycle or unexpectedly deep graph"
        ));
    }
    let manifest_path = package_root.join("Alaco.toml");
    let manifest = crate::manifest::Manifest::load(&manifest_path)?;
    let mut manifest_source = fs::read_to_string(&manifest_path)
        .map_err(|error| miette::miette!("cannot read '{}': {error}", manifest_path.display()))?;
    let package_dir = package_root.join(".alaco").join("packages");

    for (dependency_name, spec) in &manifest.dependencies {
        if let Some(local_path) = &spec.path {
            let path = package_root
                .join(local_path)
                .canonicalize()
                .map_err(|error| {
                    miette::miette!(
                        "cannot resolve local dependency '{dependency_name}' from '{}': {error}",
                        package_root.display()
                    )
                })?;
            let identity = format!("path:{}", path.display());
            if active.contains(&identity) {
                return Err(miette::miette!(
                    "cyclic package dependency detected: {} -> {dependency_name}",
                    active
                        .iter()
                        .map(|item| item
                            .trim_start_matches("git:")
                            .trim_start_matches("path:")
                            .to_owned())
                        .collect::<Vec<_>>()
                        .join(" -> ")
                ));
            }
            active.push(identity);
            let result = install_git_dependency_tree(&path, active);
            active.pop();
            result?;
            continue;
        }

        let Some(repository) = &spec.git else {
            return Err(miette::miette!(
                "dependency '{dependency_name}' in '{}' has no Git or path source; Alaco has no package registry, so use `alaco add <git-repository>` or a `path` declaration",
                manifest_path.display()
            ));
        };
        let identity = format!("git:{repository}");
        if active.contains(&identity) {
            return Err(miette::miette!(
                "cyclic Git package dependency detected: {} -> {repository}",
                active
                    .iter()
                    .map(|item| item
                        .trim_start_matches("git:")
                        .trim_start_matches("path:")
                        .to_owned())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ));
        }

        let install_name = dependency_name.replace('-', "_");
        if install_name == "std"
            || !install_name
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
            || !install_name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(miette::miette!(
                "dependency name '{dependency_name}' cannot be used as an Alaco import name"
            ));
        }
        fs::create_dir_all(&package_dir).map_err(|error| {
            miette::miette!(
                "failed to create package directory '{}': {error}",
                package_dir.display()
            )
        })?;
        let destination = package_dir.join(&install_name);
        if destination.exists() {
            return Err(miette::miette!(
                "dependency '{dependency_name}' is already present at '{}'; remove that installed package and retry",
                destination.display()
            ));
        }
        let temporary = package_dir.join(format!(
            ".alaco-dep-{}-{}",
            std::process::id(),
            install_name
        ));
        if temporary.exists() {
            fs::remove_dir_all(&temporary).map_err(|error| {
                miette::miette!("failed to clear temporary dependency directory: {error}")
            })?;
        }
        let clone = Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg("--")
            .arg(repository)
            .arg(&temporary)
            .status()
            .map_err(|error| {
                miette::miette!("failed to run Git; install Git and retry: {error}")
            })?;
        if !clone.success() {
            let _ = fs::remove_dir_all(&temporary);
            return Err(miette::miette!(
                "Git clone failed for dependency '{dependency_name}' from '{repository}'"
            ));
        }

        if let Some(rev) = &spec.rev {
            let checkout = Command::new("git")
                .arg("-C")
                .arg(&temporary)
                .args(["checkout", "--quiet", "--detach", rev])
                .status()
                .map_err(|error| {
                    miette::miette!("failed to run Git checkout for '{dependency_name}': {error}")
                })?;
            if !checkout.success() {
                let fetch = Command::new("git")
                    .arg("-C")
                    .arg(&temporary)
                    .args(["fetch", "--quiet", "origin", rev])
                    .status()
                    .map_err(|error| {
                        miette::miette!(
                            "failed to fetch Git revision '{rev}' for '{dependency_name}': {error}"
                        )
                    })?;
                if !fetch.success() {
                    let _ = fs::remove_dir_all(&temporary);
                    return Err(miette::miette!(
                        "Git revision '{rev}' for dependency '{dependency_name}' was not found or could not be fetched"
                    ));
                }
                let checkout = Command::new("git")
                    .arg("-C")
                    .arg(&temporary)
                    .args(["checkout", "--quiet", "--detach", "FETCH_HEAD"])
                    .status()
                    .map_err(|error| miette::miette!("failed to check out fetched Git revision for '{dependency_name}': {error}"))?;
                if !checkout.success() {
                    let _ = fs::remove_dir_all(&temporary);
                    return Err(miette::miette!(
                        "could not check out Git revision '{rev}' for dependency '{dependency_name}'"
                    ));
                }
            }
        }

        let child_manifest = match crate::manifest::Manifest::load(&temporary.join("Alaco.toml")) {
            Ok(manifest) => manifest,
            Err(error) => {
                let _ = fs::remove_dir_all(&temporary);
                return Err(miette::miette!(
                    "repository '{repository}' for dependency '{dependency_name}' is not an Alaco package: {error}"
                ));
            }
        };
        let actual_name = child_manifest.name.replace('-', "_");
        if actual_name != install_name {
            let _ = fs::remove_dir_all(&temporary);
            return Err(miette::miette!(
                "dependency '{dependency_name}' points to package '{}'; dependency key and package name must match after hyphen normalization",
                child_manifest.name
            ));
        }
        let revision_output = Command::new("git")
            .arg("-C")
            .arg(&temporary)
            .args(["rev-parse", "HEAD"])
            .output()
            .map_err(|error| {
                miette::miette!("failed to read Git revision for '{dependency_name}': {error}")
            })?;
        if !revision_output.status.success() {
            let _ = fs::remove_dir_all(&temporary);
            return Err(miette::miette!(
                "could not determine the Git revision for dependency '{dependency_name}'"
            ));
        }
        let revision = String::from_utf8_lossy(&revision_output.stdout)
            .trim()
            .to_owned();
        manifest_source =
            pin_git_dependency_revision(&manifest_source, dependency_name, &revision)?;

        fs::rename(&temporary, &destination).map_err(|error| {
            let _ = fs::remove_dir_all(&temporary);
            miette::miette!(
                "failed to install dependency '{dependency_name}' into '{}': {error}",
                destination.display()
            )
        })?;
        active.push(identity);
        let child_result = install_git_dependency_tree(&destination, active);
        active.pop();
        child_result?;
    }

    fs::write(&manifest_path, manifest_source).map_err(|error| {
        miette::miette!(
            "failed to pin dependency revisions in '{}': {error}",
            manifest_path.display()
        )
    })?;
    Ok(())
}

fn pin_git_dependency_revision(
    source: &str,
    dependency_name: &str,
    revision: &str,
) -> Result<String> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut in_dependencies = false;
    let mut output = lines
        .iter()
        .map(|line| (*line).to_owned())
        .collect::<Vec<_>>();
    let revision_value = toml::Value::String(revision.to_owned()).to_string();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_dependencies = trimmed == "[dependencies]";
            continue;
        }
        if !in_dependencies || !trimmed.starts_with(dependency_name) {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        if key.trim() != dependency_name || !value.contains('{') {
            continue;
        }
        let open = line.find('{').expect("checked above");
        let close = line.rfind('}').ok_or_else(|| miette::miette!("dependency '{dependency_name}' must use a single-line inline table so Alaco can pin its Git revision"))?;
        if close < open {
            return Err(miette::miette!(
                "invalid inline dependency table for '{dependency_name}'"
            ));
        }
        let table = &line[open + 1..close];
        let rev_assignment = table.match_indices("rev").find_map(|(rev_index, _)| {
            let before = table[..rev_index].trim_end();
            if !before.is_empty() && !before.ends_with(',') {
                return None;
            }
            let mut equal = rev_index + 3;
            while table
                .as_bytes()
                .get(equal)
                .is_some_and(u8::is_ascii_whitespace)
            {
                equal += 1;
            }
            (table.as_bytes().get(equal) == Some(&b'=')).then_some((rev_index, equal + 1))
        });
        if let Some((_, mut value_start)) = rev_assignment {
            while table
                .as_bytes()
                .get(value_start)
                .is_some_and(u8::is_ascii_whitespace)
            {
                value_start += 1;
            }
            let rest = &table[value_start..];
            let value_end = rest.find(',').unwrap_or(rest.len());
            let updated_table = format!(
                "{}{}{}",
                &table[..value_start],
                revision_value,
                &rest[value_end..]
            );
            output[index] = format!(
                "{}{{{}}}{}",
                &line[..open],
                updated_table,
                &line[close + 1..]
            );
            return Ok(output.join("\n") + "\n");
        }
        let updated_table = format!("{}, rev = {}", table.trim_end(), revision_value);
        output[index] = format!(
            "{}{{{}}}{}",
            &line[..open],
            updated_table,
            &line[close + 1..]
        );
        return Ok(output.join("\n") + "\n");
    }
    Err(miette::miette!(
        "dependency '{dependency_name}' was not found in [dependencies] while pinning its Git revision"
    ))
}

fn insert_dependency(source: &str, declaration: &str) -> String {
    let lines = source.lines().collect::<Vec<_>>();
    let mut section_start = None;
    let mut section_end = lines.len();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            section_start = Some(index);
            section_end = lines.len();
        } else if section_start.is_some() && trimmed.starts_with('[') && trimmed.ends_with(']') {
            section_end = index;
            break;
        }
    }

    let mut output = lines
        .iter()
        .map(|line| (*line).to_owned())
        .collect::<Vec<_>>();
    if let Some(start) = section_start {
        let mut end = section_end;
        while end > start + 1 && lines[end - 1].trim().is_empty() {
            end -= 1;
        }
        output.insert(end, declaration.to_owned());
    } else {
        if !source.is_empty() && !source.ends_with('\n') {
            output.push(String::new());
        }
        output.push("[dependencies]".to_owned());
        output.push(declaration.to_owned());
    }
    let mut result = output.join("\n");
    result.push('\n');
    result
}

fn test(path: &Path) -> Result<ExitCode> {
    let tests = discover_tests(path)?;
    if tests.is_empty() {
        return Err(miette::miette!(
            "no .aco tests found in '{}'; add test programs to tests/ or pass a test file",
            path.display()
        ));
    }

    println!("  {} {} test program(s)", bold("Testing"), tests.len());
    let started = Instant::now();
    let mut passed = 0usize;
    let mut failed = 0usize;

    for test_file in &tests {
        let label = test_file.display().to_string();
        let output = test_output_path(test_file);
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                miette::miette!(
                    "failed to create test output directory '{}': {error}",
                    parent.display()
                )
            })?;
        }

        let result = (|| -> Result<bool> {
            let source = read_source(test_file)?;
            Compiler::build(&source, &label, &output)?;
            let status = Command::new(executable_path(&output))
                .status()
                .map_err(|error| {
                    miette::miette!("failed to run test '{}': {error}", test_file.display())
                })?;
            Ok(status.success())
        })();
        cleanup(&output);

        match result {
            Ok(true) => {
                passed += 1;
                println!("  {} {label}", green("PASS"));
            }
            Ok(false) => {
                failed += 1;
                println!(
                    "  {} {label} (test program exited unsuccessfully)",
                    red("FAIL")
                );
            }
            Err(error) => {
                failed += 1;
                println!("  {} {label}", red("FAIL"));
                eprintln!("    {error:?}");
            }
        }
    }

    println!("\n  {passed} passed; {failed} failed");
    print_finished(started.elapsed());

    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn discover_tests(path: &Path) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        if path.extension().and_then(|extension| extension.to_str()) != Some("aco") {
            return Err(miette::miette!(
                "test file must use the .aco extension: '{}'",
                path.display()
            ));
        }
        return Ok(vec![path.to_path_buf()]);
    }

    if !path.is_dir() {
        return Err(miette::miette!(
            "test path '{}' does not exist or is not a directory",
            path.display()
        ));
    }

    let mut tests = fs::read_dir(path)
        .map_err(|error| {
            miette::miette!(
                "failed to read test directory '{}': {error}",
                path.display()
            )
        })?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|entry| {
            entry.is_file()
                && entry.extension().and_then(|extension| extension.to_str()) == Some("aco")
        })
        .collect::<Vec<_>>();
    tests.sort();
    Ok(tests)
}

fn test_output_path(file: &Path) -> PathBuf {
    let root = Path::new("target");
    let stem = file
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("test");
    root.join(format!(".alaco-test-{stem}-{}", std::process::id()))
}

fn clean() -> Result<ExitCode> {
    clean_target(Path::new("target"))
}

fn clean_target(target: &Path) -> Result<ExitCode> {
    if target.exists() {
        fs::remove_dir_all(target).map_err(|error| {
            miette::miette!(
                "failed to remove build directory '{}': {error}",
                target.display()
            )
        })?;
        println!("  {} Removed {}", green("✓"), target.display());
    } else {
        println!("  {} Nothing to clean", green("✓"));
    }
    Ok(ExitCode::SUCCESS)
}

fn check(file: &Path) -> Result<ExitCode> {
    let source = read_source(file)?;
    let filename = file.display().to_string();

    let start = Instant::now();

    println!("  {} {}", bold("Checking"), file.display());

    Compiler::check(&source, &filename)?;

    println!("  {} Parsed", green("✓"));
    println!("  {} Analyzed", green("✓"));

    print_finished(start.elapsed());

    Ok(ExitCode::SUCCESS)
}

fn transpile(file: &Path, requested_output: Option<&Path>) -> Result<ExitCode> {
    let source = read_source(file)?;
    let filename = file.display().to_string();
    let output = requested_output
        .map(PathBuf::from)
        .unwrap_or_else(|| file.with_extension("js"));

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            miette::miette!(
                "failed to create output directory '{}': {error}",
                parent.display()
            )
        })?;
    }

    let start = Instant::now();
    println!("  {} {}", bold("Transpiling"), file.display());
    let javascript = Compiler::transpile_javascript(&source, &filename)?;
    fs::write(&output, javascript).map_err(|error| {
        miette::miette!(
            "failed to write generated JavaScript to '{}': {error}",
            output.display()
        )
    })?;
    println!("  {} Generated JavaScript", green("✓"));
    print_finished(start.elapsed());
    println!("  {} {}", dimmed("→"), output.display());
    Ok(ExitCode::SUCCESS)
}

fn explain(file: &Path) -> Result<ExitCode> {
    let source = read_source(file)?;
    let filename = file.display().to_string();

    match Compiler::check(&source, &filename) {
        Ok(()) => {
            println!("  {} No errors found.", green("✓"));

            Ok(ExitCode::SUCCESS)
        }

        Err(error) => {
            if let Some(alaco_error) = error
                .chain()
                .find_map(|cause| cause.downcast_ref::<crate::error::AlacoError>())
                && let Some((why, fix)) = alaco_error.explanation()
            {
                eprintln!("\nWhy: {why}\nSuggestion: {fix}");
            }

            Err(error)
        }
    }
}

fn build(file: &Path, requested_output: Option<&Path>) -> Result<ExitCode> {
    let source = read_source(file)?;
    let filename = file.display().to_string();

    let output = requested_output
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output_path(file));

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            miette::miette!(
                "failed to create output directory '{}': {error}",
                parent.display()
            )
        })?;
    }

    let start = Instant::now();

    println!("  {} {}", bold("Building"), file.display());

    println!("  {} Compiling", dimmed("→"));

    let config = manifest_for_source(file)?
        .map(|manifest| manifest.build)
        .unwrap_or_default();
    Compiler::build_with_config(&source, &filename, &output, &config)?;

    println!("  {} Built", green("✓"));

    print_finished(start.elapsed());

    println!("  {} {}", dimmed("→"), output.display());

    Ok(ExitCode::SUCCESS)
}

fn run(file: &Path) -> Result<ExitCode> {
    let source = read_source(file)?;
    let filename = file.display().to_string();

    let output = temporary_output_path(file);

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            miette::miette!(
                "failed to create output directory '{}': {error}",
                parent.display()
            )
        })?;
    }

    let start = Instant::now();

    println!("  {} {}", bold("Running"), file.display());

    println!("  {} Compiling", dimmed("→"));

    let config = manifest_for_source(file)?
        .map(|manifest| manifest.build)
        .unwrap_or_default();
    if let Err(error) = Compiler::build_with_config(&source, &filename, &output, &config) {
        cleanup(&output);
        return Err(error);
    }

    println!("  {} Built", green("✓"));
    println!();

    let executable = executable_path(&output);

    let status = Command::new(&executable)
        .status()
        .map_err(|error| miette::miette!("failed to run '{}': {error}", executable.display()));

    cleanup(&output);

    let status = status?;

    println!();

    if status.success() {
        print_finished(start.elapsed());
        return Ok(ExitCode::SUCCESS);
    }

    let code = status.code().unwrap_or(1);

    Ok(ExitCode::from(code.clamp(1, 255) as u8))
}

fn read_source(file: &Path) -> Result<String> {
    if file.extension().and_then(|extension| extension.to_str()) != Some("aco") {
        return Err(miette::miette!(
            "expected an Alaco source file (.aco), got '{}'",
            file.display()
        ));
    }

    fs::read_to_string(file)
        .map_err(|error| miette::miette!("failed to read '{}': {error}", file.display()))
}

fn manifest_for_source(file: &Path) -> Result<Option<crate::manifest::Manifest>> {
    let mut directory = file.parent().unwrap_or_else(|| Path::new("."));
    loop {
        let candidate = directory.join("Alaco.toml");
        if candidate.is_file() {
            return crate::manifest::Manifest::load(&candidate).map(Some);
        }
        let Some(parent) = directory.parent() else {
            return Ok(None);
        };
        directory = parent;
    }
}

fn default_output_path(file: &Path) -> PathBuf {
    if is_project_entry(file) {
        let root = file
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| Path::new("."));
        let project_name = root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("alaco");
        return root.join("target").join(project_name);
    }

    let parent = file.parent().unwrap_or_else(|| Path::new("."));

    let name = file
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("a.out");

    parent.join(name)
}

fn temporary_output_path(file: &Path) -> PathBuf {
    if is_project_entry(file) {
        let root = file
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| Path::new("."));
        return root.join(format!("target/.alaco-run-{}", std::process::id()));
    }

    let parent = file.parent().unwrap_or_else(|| Path::new("."));

    let name = file
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("alaco_program");

    parent.join(format!(".{name}.alaco-run"))
}

fn is_project_entry(file: &Path) -> bool {
    file.file_name().is_some_and(|name| name == "main.aco")
        && file
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|name| name == "src")
}

fn executable_path(path: &Path) -> PathBuf {
    if path.components().count() == 1 {
        PathBuf::from(".").join(path)
    } else {
        path.to_path_buf()
    }
}

fn cleanup(path: &Path) {
    let _ = fs::remove_file(path);
}

fn print_finished(duration: Duration) {
    println!();

    println!(
        "  {} in {}",
        bold("Finished"),
        dimmed(format_duration(duration))
    );
}

fn format_duration(duration: Duration) -> String {
    let milliseconds = duration.as_secs_f64() * 1000.0;

    if milliseconds < 1.0 {
        format!("{milliseconds:.2}ms")
    } else if milliseconds < 1000.0 {
        format!("{milliseconds:.0}ms")
    } else {
        format!("{:.2}s", milliseconds / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        clean_target, create_project_files, default_output_path, discover_tests, format_duration,
        init_project, insert_dependency, install_git_dependency_tree, pin_git_dependency_revision,
        should_use_color, temporary_output_path, transpile,
    };
    use std::{fs, path::Path, time::Duration};

    #[test]
    fn dependency_insertion_preserves_existing_manifest_sections() {
        let original = "[package]\nname='demo'\nversion='1.0.0'\n\n[dependencies]\n# existing dependency\nlocal={ path='../local' }\n\n[build]\ncache=true\n";
        let updated = insert_dependency(
            original,
            "geometry = { git = \"https://example.com/geometry.git\", rev = \"abc123\" }",
        );
        assert!(updated.contains("# existing dependency\nlocal={ path='../local' }\ngeometry = { git = \"https://example.com/geometry.git\", rev = \"abc123\" }\n\n[build]"));
        assert!(crate::manifest::Manifest::parse(&updated, Path::new("Alaco.toml")).is_ok());
    }

    #[test]
    fn dependency_insertion_creates_a_missing_dependencies_table() {
        let updated = insert_dependency(
            "[package]\nname='demo'\nversion='1.0.0'\n",
            "geometry = { git = \"https://example.com/geometry.git\", rev = \"abc123\" }",
        );
        assert!(updated.contains("[dependencies]\ngeometry = { git = \"https://example.com/geometry.git\", rev = \"abc123\" }"));
        assert!(crate::manifest::Manifest::parse(&updated, Path::new("Alaco.toml")).is_ok());
    }

    #[test]
    fn recursive_git_dependencies_are_installed_and_pinned() {
        let root = test_directory("recursive-dependencies");
        let app = root.join("app");
        let middle = root.join("middle");
        let leaf = root.join("leaf");
        for directory in [&app, &middle, &leaf] {
            fs::create_dir_all(directory.join("src")).unwrap();
        }
        fs::write(
            leaf.join("Alaco.toml"),
            "[package]\nname='leaf'\nversion='1.0.0'\nentry='src/lib.aco'\n",
        )
        .unwrap();
        fs::write(
            leaf.join("src/lib.aco"),
            "fn answer() -> Int { return 42; }\n",
        )
        .unwrap();
        init_test_git_repo(&leaf);

        let leaf_url = leaf.to_string_lossy();
        fs::write(
            middle.join("Alaco.toml"),
            format!("[package]\nname='middle'\nversion='1.0.0'\nentry='src/lib.aco'\n\n[dependencies]\nleaf = {{ git = {leaf_url:?} }}\n"),
        ).unwrap();
        fs::write(
            middle.join("src/lib.aco"),
            "import leaf; fn get() -> Int { return leaf.answer(); }\n",
        )
        .unwrap();
        init_test_git_repo(&middle);

        let middle_url = middle.to_string_lossy();
        fs::write(
            app.join("Alaco.toml"),
            format!("[package]\nname='app'\nversion='1.0.0'\nentry='src/main.aco'\n\n[dependencies]\nmiddle = {{ git = {middle_url:?} }}\n"),
        ).unwrap();
        fs::write(
            app.join("src/main.aco"),
            "import middle; fn main() { println(middle.get()); }\n",
        )
        .unwrap();

        install_git_dependency_tree(&app, &mut Vec::new()).unwrap();

        let installed_middle = app.join(".alaco/packages/middle");
        let installed_leaf = installed_middle.join(".alaco/packages/leaf");
        assert!(installed_leaf.join("Alaco.toml").is_file());
        let app_manifest = fs::read_to_string(app.join("Alaco.toml")).unwrap();
        let middle_manifest = fs::read_to_string(installed_middle.join("Alaco.toml")).unwrap();
        assert!(app_manifest.contains(&format!("rev = \"{}\"", git_head(&installed_middle))));
        assert!(middle_manifest.contains(&format!("rev = \"{}\"", git_head(&installed_leaf))));

        let source = fs::read_to_string(app.join("src/main.aco")).unwrap();
        let generated = crate::compiler::Compiler::compile(
            &source,
            &app.join("src/main.aco").to_string_lossy(),
        )
        .expect("transitive library imports should compile");
        assert!(generated.contains("leaf::answer()"), "{generated}");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recursive_git_dependencies_reject_cycles() {
        let root = test_directory("dependency-cycle");
        let app = root.join("app");
        let alpha = root.join("alpha");
        let beta = root.join("beta");
        for directory in [&app, &alpha, &beta] {
            fs::create_dir_all(directory.join("src")).unwrap();
        }
        fs::write(
            app.join("Alaco.toml"),
            "[package]\nname='app'\nversion='1.0.0'\n",
        )
        .unwrap();
        let alpha_url = alpha.to_string_lossy();
        let beta_url = beta.to_string_lossy();
        fs::write(alpha.join("Alaco.toml"), format!("[package]\nname='alpha'\nversion='1.0.0'\n\n[dependencies]\nbeta = {{ git = {beta_url:?} }}\n")).unwrap();
        fs::write(beta.join("Alaco.toml"), format!("[package]\nname='beta'\nversion='1.0.0'\n\n[dependencies]\nalpha = {{ git = {alpha_url:?} }}\n")).unwrap();
        init_test_git_repo(&alpha);
        init_test_git_repo(&beta);
        fs::write(app.join("Alaco.toml"), format!("[package]\nname='app'\nversion='1.0.0'\n\n[dependencies]\nalpha = {{ git = {alpha_url:?} }}\n")).unwrap();

        let error = install_git_dependency_tree(&app, &mut Vec::new())
            .unwrap_err()
            .to_string();
        assert!(error.contains("cyclic Git package dependency"), "{error}");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn pinning_replaces_a_branch_revision_with_the_resolved_commit() {
        let source = "[package]\nname='demo'\nversion='1.0.0'\n\n[dependencies]\nhelper = { git = 'https://example.com/helper.git', rev='main' }\n";
        let pinned = pin_git_dependency_revision(source, "helper", "abc123def456").unwrap();
        let manifest = crate::manifest::Manifest::parse(&pinned, Path::new("Alaco.toml")).unwrap();
        assert_eq!(
            manifest.dependencies["helper"].rev.as_deref(),
            Some("abc123def456")
        );
    }

    fn init_test_git_repo(path: &Path) {
        for args in [
            vec!["init", "-q"],
            vec!["config", "user.email", "alaco-test@example.invalid"],
            vec!["config", "user.name", "Alaco Test"],
            vec!["add", "Alaco.toml", "src"],
            vec!["commit", "-qm", "test package"],
        ] {
            let output = std::process::Command::new("git")
                .current_dir(path)
                .args(args)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "Git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn git_head(path: &Path) -> String {
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    #[test]
    fn formats_short_durations_in_milliseconds() {
        assert_eq!(format_duration(Duration::from_micros(250)), "0.25ms");
        assert_eq!(format_duration(Duration::from_millis(42)), "42ms");
    }

    #[test]
    fn colors_require_interactive_output_and_respect_no_color() {
        assert!(should_use_color(true, true, false));
        assert!(!should_use_color(false, true, false));
        assert!(!should_use_color(true, false, false));
        assert!(!should_use_color(true, true, true));
    }

    #[test]
    fn formats_long_durations_in_seconds() {
        assert_eq!(format_duration(Duration::from_millis(1250)), "1.25s");
    }

    #[test]
    fn project_scaffolding_creates_a_runnable_entry_point() {
        let root = test_directory("new-project");
        fs::create_dir_all(&root).unwrap();

        create_project_files(&root).unwrap();

        let source = fs::read_to_string(root.join("src/main.aco")).unwrap();
        assert!(source.contains("fn main()"));
        assert!(source.contains("Hello, Alaco!"));
        assert!(root.join("README.md").is_file());
        assert!(root.join(".gitignore").is_file());
        let manifest = fs::read_to_string(root.join("Alaco.toml")).unwrap();
        assert!(manifest.contains("name = \""));
        assert!(manifest.contains("entry = \"src/main.aco\""));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn init_never_overwrites_an_existing_entry_point() {
        let root = test_directory("init-project");
        let source_dir = root.join("src");
        fs::create_dir_all(&source_dir).unwrap();
        let entry = source_dir.join("main.aco");
        fs::write(&entry, "// keep me\n").unwrap();

        assert!(init_project(&root).is_err());
        assert_eq!(fs::read_to_string(entry).unwrap(), "// keep me\n");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn project_initialization_preserves_existing_support_files() {
        let root = test_directory("init-preserve");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("README.md"), "my existing readme\n").unwrap();
        fs::write(root.join(".gitignore"), "custom ignore rules\n").unwrap();

        create_project_files(&root).unwrap();

        assert_eq!(
            fs::read_to_string(root.join("README.md")).unwrap(),
            "my existing readme\n"
        );
        assert_eq!(
            fs::read_to_string(root.join(".gitignore")).unwrap(),
            "custom ignore rules\n"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn project_build_outputs_live_in_target() {
        let entry = Path::new("demo/src/main.aco");
        assert_eq!(default_output_path(entry), Path::new("demo/target/demo"));
        assert!(
            temporary_output_path(entry)
                .to_string_lossy()
                .starts_with("demo/target/.alaco-run-")
        );
    }

    #[test]
    fn transpile_writes_generated_javascript_without_building_an_executable() {
        let root = test_directory("transpile");
        fs::create_dir_all(&root).unwrap();
        let source = root.join("hello.aco");
        let output = root.join("generated/hello.js");
        fs::write(&source, "fn main() { println(\"hello\"); }").unwrap();

        let result = transpile(&source, Some(&output)).unwrap();

        assert_eq!(result, std::process::ExitCode::SUCCESS);
        let generated = fs::read_to_string(&output).unwrap();
        assert!(generated.contains("function main("));
        assert!(generated.contains("const println = (...values) => console.log(...values);"));
        assert!(generated.contains("println(\"hello\")"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_discovery_finds_only_top_level_aco_files_in_sorted_order() {
        let root = test_directory("test-discovery");
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("z.aco"), "").unwrap();
        fs::write(root.join("a.aco"), "").unwrap();
        fs::write(root.join("notes.md"), "").unwrap();
        fs::write(root.join("nested/ignored.aco"), "").unwrap();

        assert_eq!(
            discover_tests(&root).unwrap(),
            vec![root.join("a.aco"), root.join("z.aco")]
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn clean_removes_only_the_given_build_directory() {
        let root = test_directory("clean");
        let target = root.join("target");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("artifact"), "generated").unwrap();
        fs::write(root.join("keep.txt"), "keep").unwrap();

        clean_target(&target).unwrap();

        assert!(!target.exists());
        assert_eq!(fs::read_to_string(root.join("keep.txt")).unwrap(), "keep");
        fs::remove_dir_all(root).unwrap();
    }

    fn test_directory(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "alaco-cli-{label}-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ))
    }
}
