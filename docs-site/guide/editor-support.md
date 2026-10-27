# Alaco Language Server

Alaco ships a standalone Language Server Protocol (LSP) server in the `alaco` executable. It uses the compiler's existing lexer and parser, and the existing semantic analyzer for diagnostics. The server speaks JSON-RPC over standard input/output; **do not use stdout for logs** when running it as an LSP.

## Start the server

Build the compiler once, then configure your editor to run:

```sh
alaco lsp
```

If you have built the compiler but have not installed it yet, use the executable directly at `target/debug/alaco lsp` (or `target/release/alaco lsp` for a release build).

## Editor features

- Lexer, parser, and semantic diagnostics on open/change/save, including Alaco diagnostic codes and source ranges.
- Completion for language keywords, primitive types, symbols in the current file, functions and structs in other `.aco` files under the workspace, standard-library modules, and common standard-library APIs.
- Hover for language constructs, user-defined functions and structs, primitive literals, I/O functions, and documented standard-library helpers.
- Go to definition, find references, document highlights, and rename for local variables and function parameters, respecting nested scopes and shadowed local names. Workspace files are also searched for top-level symbols, though global symbol identity is not fully semantic yet.
- Function signature help, parameter-name inlay hints, document/workspace symbols, folding ranges, selection ranges, and semantic tokens.
- Document/range/on-type formatting and a source action to sort top-level imports.
- Code lenses to check the current document, and links from import paths to matching local source files when those files exist in the workspace.
- Resolved completion documentation, plus the `alaco.checkWorkspace` server command to refresh diagnostics for all open documents.

The implementation is deliberately in-process and currently reparses a changed document rather than maintaining an incremental syntax tree. Definitions, references, rename, and workspace symbols scan `.aco` files beneath the workspace root on demand; unsaved open buffers override the corresponding files on disk. This avoids requiring every file to be opened, but very large workspaces may benefit from a cached index and file watching. Local bindings use scope-aware matching; top-level cross-file symbols still rely partly on lexical name matching, so same-named global functions or types are not guaranteed to resolve to a unique semantic identity. Code actions currently focus on import organization, and inlay hints show parameter names for known local function calls.

## VS Code

The **Alaco Language Support** extension provides `.aco` syntax highlighting, snippets, compiler commands, and an LSP client. It starts `alaco lsp` automatically when the language server is enabled.

### Install the v0.2.0 extension

1. Download [`alaco-language-0.2.0.vsix`](https://github.com/ManiProjs/alaco/releases/download/v0.2.0/alaco-language-0.2.0.vsix) from the [v0.2.0 release page](https://github.com/ManiProjs/alaco/releases/tag/v0.2.0).
2. In VS Code, open **Extensions**, select the **…** menu, choose **Install from VSIX…**, and select the downloaded file.
3. Install the Alaco `alaco` executable separately and ensure it is on `PATH`.
4. If the executable is not on `PATH` or has a custom name/location, set `alaco.executable` in VS Code settings to its full path. The extension adds the `lsp` subcommand itself.

The extension's `alaco.languageServer.enabled` setting can disable the language server while leaving syntax highlighting and the compiler commands available. The **Alaco: Check Current File**, **Alaco: Build Current File**, and **Alaco: Run Current File** commands use the configured executable.

For platform compiler archives and installation notes, see [Release downloads](/reference/releases).

## Neovim (built-in LSP)

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = "alaco",
  callback = function(args)
    vim.lsp.start({
      name = "alaco",
      cmd = { "alaco", "lsp" },
      root_dir = vim.fs.root(args.buf, { "Alaco.toml", ".git" }) or vim.fn.getcwd(),
    })
  end,
})
```

Make sure `.aco` files have the `alaco` filetype (for example, add `vim.filetype.add({ extension = { aco = "alaco" } })`).

The shipped plugin under `editors/nvim` configures this automatically and uses `vim.g.alaco_executable` if set.

## Vim

Install `prabirshrestha/vim-lsp` and copy `editors/vim/{ftdetect,syntax,plugin}` into your Vim runtime directory. The plugin registers `alaco lsp` for the `alaco` filetype. Set `g:alaco_executable` to override the executable path.

## Protocol details

The server supports UTF-16 LSP positions, full-document synchronization, standard Content-Length framing, and the usual `initialize`/`shutdown` lifecycle. It stores open documents in memory and sends diagnostics using `textDocument/publishDiagnostics`. It does not execute source code or invoke `clang++` during normal editor requests.
