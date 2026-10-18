# CLI reference

The compiler exposes commands for project setup, checking, building, execution, transpilation, diagnostics, and editor integration. Run `alaco --help` for the exact options supported by the executable you have installed.

| Command | Purpose |
| --- | --- |
| `alaco new <name>` | Create a project with an entry point and manifest. |
| `alaco init [directory]` | Initialize a project without overwriting an existing entry point. |
| `alaco check [file.aco]` | Parse and semantically analyze source. Defaults to the project entry point when applicable. |
| `alaco explain [file.aco]` | Check a file and display diagnostics when errors are found. |
| `alaco build [file.aco]` | Build a native executable through generated C++. |
| `alaco build [file.aco] --output <path>` | Select the native output path. |
| `alaco build [file.aco] --explain` | Explain whether a native build is reused or rebuilt. |
| `alaco build [file.aco] --no-cache` | Force a fresh native compilation. |
| `alaco run [file.aco]` | Compile and execute a native program. |
| `alaco transpile <file.aco> --target js` | Generate a JavaScript ES module; default output uses `.mjs`. |
| `alaco test [path]` | Build and run `.aco` test programs in a directory or file. |
| `alaco clean` | Remove generated project build artifacts. |
| `alaco doctor` | Check the C++ compiler and standard library setup. |
| `alaco manifest` | Validate and display package metadata. |
| `alaco add <path-or-git-url>` | Add a local or Git dependency and update `Alaco.lock`. |
| `alaco lsp` | Start the Language Server Protocol server over standard input/output. |
| `alaco --help` | Show the command list and usage. |

## Common workflows

### Create and run a project

```sh
alaco new hello
cd hello
alaco check
alaco run
alaco build
```

### Check a standalone file

```sh
alaco check examples/functions.aco
alaco run examples/functions.aco
```

### Generate JavaScript

```sh
alaco transpile examples/javascript.aco --target js
node examples/javascript.mjs
```

### Run tests

```sh
alaco test
alaco test tests/example.aco
```

Test files are compiled and executed as separate programs. A successful process exit indicates a passing test; inspect fixtures before running them because examples may touch files, use the network, or execute shell commands.

### Add a dependency

Run the command from a directory containing `Alaco.toml`:

```sh
alaco add ../shared-library
alaco add https://github.com/example/library.git --version '^1.2.0'
alaco add https://github.com/example/library.git --tag v1.2.0
```

Git version selection uses matching `vMAJOR.MINOR.PATCH` tags and records the chosen commit in `Alaco.lock`. See [Dependencies and packages](/guide/dependencies) for resolution, imports, and lockfile behavior.

### Diagnose incremental builds

```sh
alaco build --explain
alaco build --no-cache
alaco clean
```

`--explain` describes cache reuse or invalidation, `--no-cache` forces a compilation, and `clean` clears generated build and local Git checkout caches. See [Incremental builds](/guide/build-cache).

## Exit status and diagnostics

Commands signal failure through a non-success process exit status. Semantic diagnostics use stable `alaco::analysis::*` codes and include source ranges when the compiler has location information. `check` validates Alaco source, while `build` and `run` additionally depend on the generated C++ and native toolchain.
