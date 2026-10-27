# Alaco support for Visual Studio Code

This extension adds language support for Alaco (`.aco`) files, including syntax highlighting, snippets, compiler commands, and editor intelligence through Alaco's Language Server Protocol (LSP) server.

## Requirements

- Visual Studio Code 1.85 or newer.
- The `alaco` compiler executable available on `PATH`, or its absolute path configured as described below.
- A supported C++ compiler for native builds and runs. Alaco generates C++ and invokes the system compiler; `alaco doctor` can help check the toolchain.

Build Alaco from the repository root with `cargo build`, or install/use an existing Alaco executable. If building from source, the executable is at `target/debug/alaco` (or `target/release/alaco` for a release build).

## Install the extension

### Install a packaged VSIX

From this directory, package the extension with Node.js and `@vscode/vsce` available:

```sh
npx @vscode/vsce package
```

This creates an `alaco-language-<version>.vsix` file. In VS Code, open the Command Palette and run **Extensions: Install from VSIX...**, then select that file.

### Develop the extension from this repository

Open `editors/vscode` as a folder in VS Code and press **F5** to launch an Extension Development Host. Alternatively, use **Developer: Install Extension from Location...** where supported.

## Configure the compiler

By default, the extension runs `alaco` and expects it to be available on VS Code's `PATH`. If it is not, open Settings and set **Alaco: Executable** (`alaco.executable`) to the executable's absolute path, for example:

```json
{
  "alaco.executable": "/absolute/path/to/alaco"
}
```

For a compiler built in this repository, use the absolute path to `target/debug/alaco`. The extension invokes the executable directly; do not include `lsp` in the setting because the extension adds that subcommand itself.

The language server starts automatically when an Alaco file is opened. To disable LSP-based editor features, set **Alaco: Language Server: Enabled** (`alaco.languageServer.enabled`) to `false`. Syntax highlighting and the compiler commands remain available.

## Features

- Syntax highlighting for `.aco` files.
- Snippets for `main`, functions, structs, bindings, and imports.
- LSP diagnostics and completion, hover information, go to definition, references, rename, formatting, semantic tokens, code actions, inlay hints, code lenses, and import links where supported by the server.
- An **Alaco** output channel for compiler output and language-server logs.

## Commands

Open the Command Palette (`Ctrl+Shift+P` on Windows/Linux or `Cmd+Shift+P` on macOS) and search for these commands:

| Command | Behavior |
| --- | --- |
| **Alaco: Check Current File** | Checks the active `.aco` file without building an executable. |
| **Alaco: Build Project** | Builds the project from the current workspace folder, using its `Alaco.toml` entry point. |
| **Alaco: Run Current File** | Runs the project from the current workspace folder, using its `Alaco.toml` entry point. |

When an `.aco` file is active, the editor title toolbar also shows a **Play** button for **Alaco: Run Current File**.

For **Build Project** and **Run Current File**, open the project directory—the one containing `Alaco.toml`—as the VS Code workspace folder. These commands intentionally operate on the project entry point rather than the currently active source file. To create a project, run `alaco new <name>` in a terminal and open the resulting directory. The entry point defaults to `src/main.aco` and can be configured in `Alaco.toml`.

The extension writes command output to the **Alaco** output channel. Use **View: Toggle Output** and select **Alaco** if you need to inspect compiler output or an error.

## Troubleshooting

- **Alaco executable could not be started:** Check `alaco.executable`, ensure the file exists and is executable, and restart VS Code after changing your shell's `PATH`.
- **Language server did not start:** Verify that the configured executable can run `alaco lsp` from a terminal. Check the **Alaco** output channel for details.
- **Native build fails:** Install/configure a supported C++ compiler and run `alaco doctor` from a terminal.
- **Build or Run uses the wrong project:** Open the directory containing the intended `Alaco.toml` as the workspace folder, and verify its `entry` setting.

For language and project details, see the [Alaco editor support overview](../README.md), the [Language Server documentation](../../docs/lsp.md), and the [project workflow guide](../../docs/project-workflow.md).
