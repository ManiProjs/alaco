# Alaco

Alaco (also called **A**) is a hobby programming language implemented in Rust. The compiler parses and analyzes `.aco` source files, generates C++, and invokes a system C++ compiler to build a native executable.

## Quick start

Install Rust and a C++ compiler supported by your platform, then build the compiler from this repository:

```sh
cargo build
```

Create `hello.aco`:

```alaco
fn main() {
    println("Hello, Alaco!");
}
```

Run or check it with Cargo:

```sh
cargo run -- run hello.aco
cargo run -- check hello.aco
```

When using an installed `alaco` executable, use the same commands directly. The `alaco` command is the entry point for the full toolchain.

## CLI reference

| Command | Description |
| --- | --- |
| `alaco new <name>` | Create a new project with a working entry point. |
| `alaco init [directory]` | Initialize a project in an existing directory without overwriting an entry point. |
| `alaco check [file.aco]` | Parse and analyze a source file. Defaults to `src/main.aco` in a project. |
| `alaco transpile [file.aco]` | Generate JavaScript source without building a native executable. Defaults to a sibling `.js` file. |
| `alaco transpile [file.aco] --output <path>` | Choose the generated JavaScript output path. |
| `alaco explain [file.aco]` | Check a file and show diagnostics when errors are found. |
| `alaco build [file.aco]` | Build a native executable. Project builds go in `target/`. |
| `alaco build [file.aco] --output <path>` | Build to a specified output path. |
| `alaco run [file.aco]` | Compile and run a program, cleaning up the temporary executable. |
| `alaco test [path]` | Build and run each `.aco` test program in `tests/`, or in a supplied file/directory. |
| `alaco clean` | Remove generated artifacts in the current project's `target/` directory. |
| `alaco doctor` | Check for the C++ compiler and installed standard library. |
| `alaco manifest` | Validate and display `Alaco.toml` package metadata. |
| `alaco add <git-repo>` | Install a library from Git and add it to `Alaco.toml`. |
| `alaco lsp` | Start the Language Server Protocol server over stdio for editor features. |
| `alaco --help` | Show command-line help. |

For a quick start:

```sh
alaco new hello
cd hello
alaco run
alaco check
alaco build
alaco test
alaco doctor
```

Inside a project, `Alaco.toml` defines package metadata and the default entry point. `alaco new` and `alaco init` create a manifest. Pass a `.aco` path to run a standalone file instead.

### Package manifests

Every Alaco project uses `Alaco.toml`:

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2026"
entry = "src/main.aco"

[dependencies]
# Git dependencies are added with `alaco add <git-repo>`.
# my_utils = { git = "https://example.com/my_utils.git", rev = "<commit>" }
# Local packages are also supported: my_utils = { path = "../my_utils" }

[build]
target = "native"       # or a compiler target triple, e.g. "aarch64-unknown-linux-gnu"
backend = "cpp"
compiler = "clang++"
flags = ["-O2"]
cache = true
source-map = false
```

`name` and a semantic `version` are required. `edition` currently supports `2026`; `entry` defaults to `src/main.aco` and must be a relative `.aco` path inside the package. Run `alaco manifest` to validate and inspect the manifest. Run `alaco add <git-repo>` from a project directory to clone a repository containing `Alaco.toml`; Alaco installs it under `.alaco/packages/`, pins the resolved Git commit in `Alaco.toml`, and recursively installs that library's Git and local-path dependencies. Every Git dependency is pinned to its resolved commit SHA, including transitive dependencies. Import the package entry with `import package_name;`, or a module with `import package_name.module;`. Hyphens in package names are normalized to underscores for the dependency/import name. There is no registry: version-only declarations without a Git or path source produce an error. Cyclic package dependency graphs are rejected with a diagnostic.

The optional `[build]` table controls native compilation. `target = "native"` uses the host compiler defaults; another value is passed to Clang as `--target=...`. Only the `cpp` backend is supported. `compiler` selects the C++ compiler executable, and each string in `flags` is passed as a separate compiler argument. `cache` enables a local content-addressed cache under `.alaco-cache` beside the output. Alaco still parses, analyzes, and generates C++ on each invocation; when all inputs match, the cache skips the native compile/link step and reuses the executable. Cache keys include source and module content, standard-library files, compiler identity/version, target, and flags. `source-map = true` preserves the generated `.cpp` file and writes a `.cpp.map.json` sidecar. The sidecar maps generated C++ line numbers to Alaco source paths and line numbers wherever C++ `#line` directives are present. Currently, directives cover statements in the main source file; imported-module statements and generated code outside mapped statements have no source mapping. Cross-compilation requires a compiler that supports the requested target and access to that target's headers, libraries, and runtime. There is no dedicated sysroot setting; pass a compiler option such as `--sysroot=/path/to/sysroot` through `flags` when needed. A target triple alone does not provide a sysroot or cross-compilation toolchain.

### Tests

Put test programs in `tests/` with one `.aco` file per test. Each file is compiled and run as a separate program; a successful process exit means the test passed. A test can also be run directly with `alaco test tests/example.aco`. Tests in the supplied directory are discovered one level deep and run in filename order. Use `alaco clean` to remove generated build artifacts.

## Documentation

- [Language guide](docs/language-guide.md) — syntax, types, functions, control flow, structs, imports, and shell expressions.
- [Standard library reference](docs/standard-library.md) — module APIs, return values, and runtime limitations.
- [Project workflow](docs/project-workflow.md) — create a project, use the check-run-build cycle, resolve local modules, and interpret diagnostics.
- [Examples catalog](examples/README.md) — runnable examples grouped by language feature, module, and package workflow, including an intentional diagnostic fixture.
- [Editor support](editors/README.md) — VS Code, Neovim, and Vim syntax highlighting, compiler commands, and LSP integration.
- [Language Server](docs/lsp.md) — LSP setup, editor features, and current implementation boundaries.
- [AlacoUp release metadata](docs/alacoup-releases.md) — stable manifest URL, archive variants, target triples, and checksum format.

## Standard library example

```alaco
import std.math;
import std.string;

fn main() {
    println(math.clamp(math.abs(-12), 0, 10));
    println(string.concat("Ala", "co"));
    println(string.repeat_string("ha", 3));
}
```

`std.io` is available by default. Other modules must be imported explicitly. See the full [standard-library reference](docs/standard-library.md) for all currently documented modules.

The standard library includes integer math, string composition, statistics, unit conversion, validation, filesystem, environment, path, JSON text, basic TOML syntax checking, HTTP GET/JSON POST, and timing helpers. The JSON/TOML/HTTP APIs are intentionally small today; see the reference for their limitations. The language also supports `previous name` for the value before an assignment and synchronous `when condition { ... }` checks within a lexical block.

## Compiler diagnostics

The analyzer checks types before C++ generation and can suggest corrections for misspelled function names, module functions, and struct fields. Semantic diagnostics use stable `alaco::analysis::*` codes and highlight the relevant expression, declaration, parameter, field, or type name where source information is available. Missing-entry-point errors cannot point to a specific source span.

## Examples and tests

The [`examples/`](examples/) directory contains small programs and standard-library integration examples. Run the compiler's test suite with:

```sh
cargo test
```
