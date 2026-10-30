# Release downloads

The current stable release is **Alaco v0.2.0**, published on October 27, 2026.

The recommended way to install and manage Alaco compiler versions is [AlacoUp](https://github.com/ManiProjs/alacoup). AlacoUp is a toolchain manager: it downloads and installs published Alaco releases, keeps multiple versions, switches the active version, and removes inactive versions. It does not replace the Alaco compiler or define the language's project commands. Its bootstrap installer installs AlacoUp and then runs `alacoup install` to install the latest stable Alaco release and standard library. Rust and Cargo are not required. Follow the [AlacoUp installation instructions](https://github.com/ManiProjs/alacoup#install-alacoup-and-alaco) for macOS, Linux, and Windows.

Use `alacoup install` for the latest stable compiler, `alacoup install 0.2.0` for an exact version, `alacoup list` to see installed versions, and `alacoup use VERSION` to switch versions. Once a version is active, use the `alaco` command for compiler and project operations such as `alaco new`, `alaco check`, `alaco run`, and `alaco build`.

- [Open the v0.2.0 GitHub Release](https://github.com/ManiProjs/alaco/releases/tag/v0.2.0) for all downloads, checksums, and installation notes.
- [Download the VS Code extension (`alaco-language-0.2.0.vsix`)](https://github.com/ManiProjs/alaco/releases/download/v0.2.0/alaco-language-0.2.0.vsix).

## Available compiler archives

These archives are the Alaco compiler releases consumed by AlacoUp. They are also available for users who prefer to install the compiler manually.

Release CI publishes binary-only and combined archives for these target platforms:

| Target | Platform |
| --- | --- |
| `x86_64-unknown-linux-gnu` | Linux x86-64 (GNU libc) |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-apple-darwin` | macOS Apple Silicon |
| `x86_64-pc-windows-msvc` | Windows x86-64 (MSVC) |

For each target, choose the archive ending in `-binary.tar.gz` for the compiler executable alone, or `.tar.gz` without `-binary` for a bundle that includes the standard library. A separate `alaco-0.2.0-stdlib.tar.gz` archive contains only the standard library. All archives include an `INSTALL.txt` file with setup guidance.

The binary-only distribution requires a separately installed standard library, configured with `ALACO_STDLIB_DIR`. The combined archive contains `std/`; point `ALACO_STDLIB_DIR` at the extracted bundle root or install that directory in the default per-user location described in `INSTALL.txt`. Building native programs also requires `clang++` on `PATH`.

## Install the VS Code extension

The VSIX contains the Alaco Language Support extension, including syntax highlighting, snippets, compiler commands, and the language-server client. It does **not** bundle the Alaco compiler executable.

1. Download the VSIX linked above.
2. Open VS Code's **Extensions** view and select **… → Install from VSIX…**.
3. Select `alaco-language-0.2.0.vsix`.
4. Install the Alaco compiler and ensure `alaco` is on `PATH`, or configure `alaco.executable` with the full path to the executable.

See [Editor support & LSP](/guide/editor-support) for editor features and setup details.

## Integrity and update metadata

The release includes `SHA256SUMS` for the published `.tar.gz` and `.vsix` artifacts, plus `alacoup-manifest.json` for AlacoUp. Use the checksums to verify downloaded archives before installing them. The manifest's URLs point to the matching GitHub release assets.
