# Alaco project workflow

This page describes a practical edit-check-run cycle for the current Alaco toolchain. For language syntax, see the [language guide](language-guide.md); for standard-library APIs, see the [standard-library reference](standard-library.md).

## 1. Check the toolchain

Build the compiler from the repository:

```sh
cargo build
```

Alaco generates C++ and invokes a system C++ compiler to produce native executables. If a build fails before Alaco reports a source-language error, check that a supported C++ toolchain is installed. `alaco doctor` checks the compiler and standard-library setup when using an installed CLI.

## 2. Create a project

Create a scaffold with:

```sh
alaco new hello
cd hello
```

To initialize an existing directory instead, run `alaco init` from that directory, or pass a target directory to `alaco init path`. The project manifest is `Alaco.toml`; the default entry point is `src/main.aco`.

A minimal manifest looks like this:

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2026"
entry = "src/main.aco"

[dependencies]
```

The package name and semantic version are required. The current supported edition is `2026`. The entry path must be a relative `.aco` path within the package.

## 3. Add a library from Git

From the project root, run:

```sh
alaco add https://github.com/example/geometry-alaco.git
```

The repository must contain a valid `Alaco.toml`. Alaco clones it into `.alaco/packages/<name>`, reads its package name, and adds a Git dependency pinned to the current commit in your project's `Alaco.toml`:

```toml
[dependencies]
geometry = { git = "https://github.com/example/geometry-alaco.git", rev = "<commit-hash>" }
```

The package can then be imported by its dependency name. `import geometry;` loads the dependency's configured `entry` file (library authors should set `entry = "src/lib.aco"`). `import geometry.shapes;` loads `shapes.aco` from the package root, or `src/shapes.aco` when the root-level module is absent. Hyphens in package names are converted to underscores for import names. Alaco uses Git directly; there is no package registry. Running `alaco add` recursively installs Git and local-path dependencies declared by each library. Git dependencies are pinned to the resolved commit SHA in each package's manifest. Cyclic dependency graphs are rejected. Version-only declarations without a Git or path source are not fetchable and produce an error.

To refresh a dependency to a newer commit, remove its manifest entry and installed `.alaco/packages/<name>` directory, then run `alaco add <git-repository>` again.

## 4. Use the check-run-build cycle

```sh
alaco check src/main.aco
alaco run src/main.aco
alaco build src/main.aco
```

- `check` parses and analyzes the program without producing a native executable.
- `run` compiles and executes the program, then cleans up its temporary executable.
- `build` produces a native executable; use `--output path` to choose the output path.
- `test` discovers and runs `.aco` test programs under `tests/`, or under a supplied file/directory.
- `clean` removes generated artifacts from the current project's `target/` directory.

Within a project, commands that accept an optional source path use the configured entry point where applicable. Run `alaco --help` for the installed version's full option list.

## 5. Organize local modules

Imports are resolved relative to the file containing the import:

```text
src/
  main.aco
  helpers.aco
  helpers/
    greetings.aco
```

From `src/main.aco`, `import helpers;` resolves to `src/helpers.aco`, while `import helpers.greetings;` resolves to `src/helpers/greetings.aco`. A module can be renamed with `as`, or one function can be brought into the current namespace with `import hello from helpers;`.

Keep the source files consistent with the import paths. Dotted imports require matching directories and `.aco` files. Cyclic imports and duplicate aliases are reported as errors.

## 6. Read diagnostics

The compiler's semantic diagnostics use `alaco::analysis::*` codes and, where possible, highlight the source expression or declaration responsible for the problem. Typical errors include unknown functions, incorrect argument types, unknown types, duplicate declarations, invalid struct fields, and missing entry points.

For an example of a deliberate error, run:

```sh
alaco check examples/a_file_with_error.aco
```

This file intentionally calls `printl` instead of a known output function. The command should fail; that failure demonstrates the name-analysis diagnostic rather than a broken test setup.

## 7. Run examples and tests safely

The [`examples/`](../examples/README.md) catalog describes the supplied programs. Most are standalone demonstrations, but some have external effects:

- `http_demo.aco` makes a network request and needs network access.
- `runtime_stdlib.aco` creates and removes files and directories in its working directory.
- `shell.aco` executes a host command.
- `a_file_with_error.aco` is intentionally invalid and should fail checking.

Read an example before running it, especially when it accesses the filesystem, environment, network, or shell. Use `cargo test` to run the compiler's Rust test suite.

## Current implementation boundaries

Alaco is evolving. The standard library's JSON helpers expose a limited set of top-level typed lookups; TOML support covers a subset rather than the full TOML 1.0 grammar; HTTP calls do not expose status codes or headers; and interpolation currently accepts variable names rather than arbitrary expressions. These limitations are documented in the language and standard-library references.
