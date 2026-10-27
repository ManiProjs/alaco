# Alaco v0.2.0

Alaco v0.2.0 improves the language tooling and runtime, and publishes the VS Code extension alongside the cross-platform release archives.

## Highlights

- **Smarter language server:** completion now includes functions and structs from other workspace files. Go to Definition, Find References, document highlighting, and Rename resolve local variables and function parameters with scope awareness, including shadowed names.
- **JavaScript transpilation runtime:** adds the JavaScript transpilation runtime and related runtime helpers.
- **Improved type checking:** expands type checking and callable type support.
- **Native build pipeline:** improves runtime helpers and the native build pipeline.
- **VS Code support:** includes the Alaco Language Support extension package (`alaco-language-0.2.0.vsix`) in the GitHub Release.
- **Release downloads:** provides platform-specific binary and combined standard-library archives, a standard-library-only archive, SHA-256 checksums, and the AlacoUp release manifest.

## Downloads

Download the archive matching your operating system and architecture. The combined archives include the standard library; binary-only archives require installing the standard library separately. Compiling Alaco programs requires `clang++`.

Install the VS Code extension by opening the Extensions view, selecting **Install from VSIX…**, and choosing `alaco-language-0.2.0.vsix`.

See the [README](README.md) and documentation for setup details and language usage.
