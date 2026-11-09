//! Deterministic lockfile generation for the currently supported Git and path
//! package sources. Version-only dependencies require a registry, which Alaco
//! does not provide yet, and are rejected rather than silently left unlocked.

use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use miette::Result;

use crate::manifest::{DependencySpec, Manifest};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LockedPackage {
    name: String,
    version: String,
    source: String,
    revision: String,
    dependencies: Vec<String>,
}

pub fn write_for_current_package(check_only: bool) -> Result<ExitCode> {
    let root = std::env::current_dir()
        .map_err(|error| miette::miette!("cannot determine current directory: {error}"))?;
    let manifest_path = root.join("Alaco.toml");
    let manifest = Manifest::load(&manifest_path)?;
    let packages = resolve_graph(&root, &manifest)?;
    let contents = render_lockfile(&packages);
    let destination = root.join("Alaco.lock");
    if check_only {
        let existing = std::fs::read_to_string(&destination).map_err(|error| {
            miette::miette!(
                "cannot read '{}': {error}; run `alaco lock`",
                destination.display()
            )
        })?;
        if existing != contents {
            eprintln!("  Alaco.lock is out of date; run `alaco lock` to refresh it");
            return Ok(ExitCode::FAILURE);
        }
        println!("  Alaco.lock is up to date");
        return Ok(ExitCode::SUCCESS);
    }
    let temporary = root.join(format!(".Alaco.lock.{}.tmp", std::process::id()));
    std::fs::write(&temporary, contents)
        .map_err(|error| miette::miette!("cannot write temporary lockfile: {error}"))?;
    if let Err(error) = std::fs::rename(&temporary, &destination) {
        let _ = std::fs::remove_file(&temporary);
        return Err(miette::miette!(
            "cannot replace '{}': {error}",
            destination.display()
        ));
    }
    println!(
        "  Locked {} package(s) in {}",
        packages.len(),
        destination.display()
    );
    Ok(ExitCode::SUCCESS)
}

pub fn verify_if_present(root: &Path, manifest: &Manifest) -> Result<()> {
    let path = root.join("Alaco.lock");
    if !path.is_file() {
        return Ok(());
    }
    let existing = std::fs::read_to_string(&path)
        .map_err(|error| miette::miette!("cannot read '{}': {error}", path.display()))?;
    let expected = render_lockfile(&resolve_graph(root, manifest)?);
    if existing != expected {
        return Err(miette::miette!(
            "Alaco.lock does not match Alaco.toml or the installed dependency revisions; run `alaco lock`"
        ));
    }
    Ok(())
}

fn resolve_graph(root: &Path, manifest: &Manifest) -> Result<Vec<LockedPackage>> {
    let mut packages = BTreeMap::<String, LockedPackage>::new();
    let mut visiting = HashSet::<PathBuf>::new();
    visit_package(root, root, manifest, &mut packages, &mut visiting)?;
    Ok(packages.into_values().collect())
}

fn visit_package(
    workspace_root: &Path,
    package_root: &Path,
    manifest: &Manifest,
    packages: &mut BTreeMap<String, LockedPackage>,
    visiting: &mut HashSet<PathBuf>,
) -> Result<()> {
    let canonical = package_root.canonicalize().map_err(|error| {
        miette::miette!(
            "cannot resolve package directory '{}': {error}",
            package_root.display()
        )
    })?;
    if !visiting.insert(canonical.clone()) {
        return Err(miette::miette!(
            "cyclic package dependency detected while locking '{}'",
            package_root.display()
        ));
    }

    for (dependency_name, spec) in &manifest.dependencies {
        let dependency_root = dependency_directory(package_root, dependency_name, spec)?;
        let dependency_manifest_path = dependency_root.join("Alaco.toml");
        let dependency_manifest = Manifest::load(&dependency_manifest_path)?;
        let (source, revision) = match spec {
            DependencySpec {
                git: Some(url),
                rev,
                ..
            } => {
                let revision = rev
                    .as_deref()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| {
                        miette::miette!(
                            "Git dependency '{}' has no pinned revision; run `alaco add` again",
                            dependency_name
                        )
                    })?;
                let actual = git_revision(&dependency_root)?;
                if actual != revision {
                    return Err(miette::miette!(
                        "Git dependency '{}' is checked out at {}, but Alaco.toml pins {}; reinstall it before locking",
                        dependency_name,
                        actual,
                        revision
                    ));
                }
                (format!("git+{url}"), revision.to_owned())
            }
            DependencySpec {
                path: Some(path), ..
            } => {
                let display = dependency_root
                    .strip_prefix(workspace_root)
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|_| PathBuf::from(path));
                (format!("path+{}", display.to_string_lossy()), String::new())
            }
            _ => {
                return Err(miette::miette!(
                    "dependency '{}' uses a version requirement, but no Alaco package registry is configured; use a Git or path dependency",
                    dependency_name
                ));
            }
        };

        let nested_names = dependency_manifest
            .dependencies
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let key = format!(
            "{}@{}#{source}",
            dependency_manifest.name, dependency_manifest.version
        );
        packages.entry(key).or_insert_with(|| LockedPackage {
            name: dependency_manifest.name.clone(),
            version: dependency_manifest.version.clone(),
            source,
            revision,
            dependencies: nested_names,
        });
        visit_package(
            workspace_root,
            &dependency_root,
            &dependency_manifest,
            packages,
            visiting,
        )?;
    }
    visiting.remove(&canonical);
    Ok(())
}

fn dependency_directory(package_root: &Path, name: &str, spec: &DependencySpec) -> Result<PathBuf> {
    let directory = if let Some(path) = &spec.path {
        package_root.join(path)
    } else if spec.git.is_some() {
        package_root
            .join(".alaco")
            .join("packages")
            .join(name.replace('-', "_"))
    } else {
        return Err(miette::miette!(
            "dependency '{}' cannot be locked without a Git URL or local path",
            name
        ));
    };
    if !directory.join("Alaco.toml").is_file() {
        return Err(miette::miette!(
            "dependency '{}' is not installed at '{}'; install it before running `alaco lock`",
            name,
            directory.display()
        ));
    }
    Ok(directory)
}

fn git_revision(directory: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| miette::miette!("failed to inspect Git dependency: {error}"))?;
    if !output.status.success() {
        return Err(miette::miette!(
            "'{}' is declared as a Git dependency but is not a Git checkout",
            directory.display()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn render_lockfile(packages: &[LockedPackage]) -> String {
    let mut root = toml::map::Map::new();
    root.insert("lock-version".into(), toml::Value::Integer(1));
    let entries = packages
        .iter()
        .map(|package| {
            let mut table = toml::map::Map::new();
            table.insert("name".into(), toml::Value::String(package.name.clone()));
            table.insert(
                "version".into(),
                toml::Value::String(package.version.clone()),
            );
            table.insert("source".into(), toml::Value::String(package.source.clone()));
            if !package.revision.is_empty() {
                table.insert(
                    "revision".into(),
                    toml::Value::String(package.revision.clone()),
                );
            }
            table.insert(
                "dependencies".into(),
                toml::Value::Array(
                    package
                        .dependencies
                        .iter()
                        .cloned()
                        .map(toml::Value::String)
                        .collect(),
                ),
            );
            toml::Value::Table(table)
        })
        .collect();
    root.insert("package".into(), toml::Value::Array(entries));
    toml::to_string(&toml::Value::Table(root)).expect("lockfile values are serializable")
}

#[cfg(test)]
mod tests {
    use super::{LockedPackage, render_lockfile, resolve_graph, verify_if_present};
    use std::fs;

    #[test]
    fn lockfile_rendering_is_deterministic_and_keeps_git_revisions() {
        let package = LockedPackage {
            name: "geometry".into(),
            version: "1.2.3".into(),
            source: "git+https://example.invalid/geometry".into(),
            revision: "0123456789abcdef".into(),
            dependencies: vec!["math".into()],
        };
        let first = render_lockfile(std::slice::from_ref(&package));
        let second = render_lockfile(&[package]);
        assert_eq!(first, second);
        let parsed: toml::Value = toml::from_str(&first).unwrap();
        assert_eq!(parsed["lock-version"].as_integer(), Some(1));
        assert_eq!(
            parsed["package"][0]["revision"].as_str(),
            Some("0123456789abcdef")
        );
    }

    #[test]
    fn lockfile_resolves_local_dependency_graphs_deterministically() {
        let root = std::env::temp_dir().join(format!("alaco-lock-{}", std::process::id()));
        let dependency = root.join("vendor/utility");
        fs::create_dir_all(dependency.join("src")).unwrap();
        fs::write(
            root.join("Alaco.toml"),
            "[package]\nname='app'\nversion='1.0.0'\n[dependencies]\nutility={ path='vendor/utility' }\n",
        )
        .unwrap();
        fs::write(
            dependency.join("Alaco.toml"),
            "[package]\nname='utility'\nversion='2.1.0'\n",
        )
        .unwrap();
        let manifest = crate::manifest::Manifest::load(&root.join("Alaco.toml")).unwrap();
        let packages = resolve_graph(&root, &manifest).unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].name, "utility");
        assert!(packages[0].source.contains("vendor/utility"));
        fs::write(root.join("Alaco.lock"), render_lockfile(&packages)).unwrap();
        verify_if_present(&root, &manifest).expect("fresh lockfile should validate");
        fs::write(
            dependency.join("Alaco.toml"),
            "[package]\nname='utility'\nversion='2.2.0'\n",
        )
        .unwrap();
        assert!(verify_if_present(&root, &manifest).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
