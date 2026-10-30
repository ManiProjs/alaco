# Troubleshooting

This page is organized by the symptom you see, not by the compiler subsystem you need to know. Start with the first failing command, keep its complete output, and follow only the section that matches the failure. If the problem is a source-language error, use [Diagnostics](/guide/diagnostics) for the compiler-stage model and source locations.

## First response: establish what is installed

Before changing project files, confirm which compiler is running and whether its native toolchain is available:

```sh
alaco --version
alaco --help
alaco doctor
alacoup doctor
```

AlacoUp manages compiler installations; `alaco` is the compiler and project command. If `alaco` is not found, check that the AlacoUp bin directory is on `PATH` and open a new terminal. On the default installation this directory is `~/.alacoup/bin`.

If multiple compiler versions are installed, inspect them with `alacoup list` and select the intended version with `alacoup use VERSION`. See [Release downloads](/reference/releases) for installation and version-management details.

## `alaco` or `alacoup` is not found

1. Confirm the installation completed without an error.
2. Check whether `~/.alacoup/bin` is in the current shell's `PATH`.
3. Open a new terminal after changing shell startup files.
4. Run `alacoup --version`, then `alaco --version`.

If AlacoUp itself is unavailable, use the installation instructions linked from [Release downloads](/reference/releases). Do not try to fix a missing compiler by installing Rust: published Alaco releases do not require a Rust toolchain.

## `alaco check` fails

`check` validates Alaco syntax, names, and types. It does not invoke the native C++ compiler. Classify the diagnostic before editing code:

| Diagnostic symptom | Inspect |
| --- | --- |
| Unexpected token, missing delimiter, malformed block | Braces, parentheses, commas, statement endings, and whether the syntax is implemented. |
| Unknown function, type, module, or field | Spelling, declaration scope, imports, aliases, and module file paths. |
| Wrong argument count or incompatible type | Function signature, argument order, annotations, and the actual inferred types. |
| Duplicate declaration or import alias | Names already declared in the same scope and aliases introduced by imports. |
| Missing entry point | The selected source file or project's configured `entry` path and its `main` function. |

Reduce the program to the smallest file that still fails. If an import is involved, temporarily replace imported calls with a small local declaration to determine whether the problem is in the importing file or the imported module. The [Diagnostics guide](/guide/diagnostics) explains source spans and analysis stages.

## `check` succeeds, but `build` or `run` fails

This usually means the failure is beyond Alaco's front end. Native builds generate C++ and invoke `clang++`:

```sh
alaco doctor
alacoup doctor
command -v clang++
alaco build src/main.aco
```

On Windows, use the supported MSVC toolchain described by the release installation notes rather than assuming a Unix `clang++` command is available. On macOS, install or select a working Xcode Command Line Tools toolchain if `clang++` is missing. On Linux, install the C++ compiler and development libraries required by the program.

Read the **first** native compiler error. Later diagnostics often cascade from the first missing header, unsupported generated construct, or linker failure. If the program uses native HTTP on Unix-like systems, check the OpenSSL development headers and libraries as described in the [standard-library reference](/reference/standard-library).

## The program builds, but behaves incorrectly

Separate compiler behavior from runtime input and operating-system effects:

1. Run `alaco check` again to make sure the source being tested is the source you intended.
2. Reproduce the issue with the smallest input and fewest source files possible.
3. Print the values immediately before the operation that behaves unexpectedly.
4. Check relative paths against the process working directory and the file containing any import; imports are resolved relative to their source file.
5. Check return values from APIs that report failure with `Bool`, `-1`, or an empty string.

An empty string is not always proof of success: some file and HTTP helpers also return `""` on failure. See the API-specific notes in the [standard-library reference](/reference/standard-library).

## An import or dependency cannot be resolved

First determine whether the import is local or package-based.

**Local imports** are relative to the source file containing the import. For `import helpers.greetings;` from `src/main.aco`, check for `src/helpers/greetings.aco`, including exact capitalization. Check for cyclic imports and duplicate aliases if the files exist.

**Package imports** require a matching entry in `[dependencies]` in `Alaco.toml`. A package's root import resolves to `src/lib.aco`; dotted imports resolve below the dependency's `src/` directory. Alaco does not have a central package registry or an `alaco add` command.

For Git dependencies, verify the repository URL, network access, credentials, and version tag. Automatic version selection recognizes `vMAJOR.MINOR.PATCH` tags; prerelease ordering is not implemented. Inspect `Alaco.lock` if a previously selected commit keeps being used. See [Dependencies and packages](/guide/dependencies), the [manifest reference](/reference/manifest), and the [lockfile reference](/reference/lockfile).

## A native build seems stale or inconsistent

Alaco does not currently expose `--no-cache` or a cache-explanation flag. Remove generated project artifacts and rebuild:

```sh
alaco clean
alaco build src/main.aco
```

`alaco clean` removes generated artifacts under the current project's `target/` directory. It also removes the project-local Git dependency checkout cache, so the next build may need network access. It does not delete source files, `Alaco.toml`, or `Alaco.lock`. See [Build artifacts and troubleshooting](/guide/build-cache).

## Generated JavaScript behaves differently from native output

The JavaScript backend is not fully runtime-compatible with the native C++ backend. Check the [Compiler targets guide](/guide/targets) and [Current limitations](/reference/limitations), then run the generated file directly with Node.js:

```sh
alaco transpile src/main.aco --output build/main.js
node build/main.js
```

Timed variables and shell expressions are not supported by JavaScript output. JavaScript HTTP helpers invoke the external `curl` command, which must be on `PATH`. Keep a small test for every target-specific feature your program relies on.

## The editor shows errors, but the command-line check succeeds

Confirm that the editor and terminal use the same compiler executable and version. In VS Code, inspect `alaco.executable`; otherwise ensure the editor process inherits the expected `PATH`. Restart the language server after changing the executable or compiler version.

The language server analyzes open documents and workspace files, but some cross-file symbol operations are still based partly on lexical matching. A navigation or rename result can therefore be less precise than a compiler diagnostic. The server communicates over stdout/stdin, so wrappers must not print logs to stdout. See [Editor support & LSP](/guide/editor-support).

## Prepare a useful bug report

Include enough information for another person to reproduce the failure:

1. Alaco version (`alaco --version`) and host operating system/architecture.
2. The exact command, run from the directory where it failed.
3. The complete first useful error, including the compiler stage if known.
4. A minimal `.aco` example and the smallest relevant `Alaco.toml` / import layout.
5. Whether `alaco check` succeeds and whether the failure is native-only or also occurs in JavaScript output.
6. Relevant `alacoup doctor` output when the toolchain is involved.

Remove API keys, access tokens, private paths, and other sensitive data before sharing logs. Avoid sending an entire project when a small reproducer is enough.

## Related pages

- [Diagnostics and compiler stages](/guide/diagnostics)
- [CLI reference](/reference/cli)
- [Current limitations](/reference/limitations)
- [Standard-library reference](/reference/standard-library)
- [Project workflow](/guide/project-workflow)
