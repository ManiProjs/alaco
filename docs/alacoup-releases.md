# AlacoUp release metadata

Alaco's version-tag release workflow publishes a machine-readable manifest for
AlacoUp, the Alaco toolchain version manager.

## Stable manifest URL

AlacoUp can discover the latest stable release from:

`https://github.com/ManiProjs/alaco/releases/latest/download/alacoup-manifest.json`

Each release also contains a version-pinned manifest at:

`https://github.com/ManiProjs/alaco/releases/download/vVERSION/alacoup-manifest.json`

Replace `VERSION` with the full tag version, such as `0.1.0` (the tag includes
the leading `v`). The latest URL follows GitHub's latest non-prerelease release.

## Manifest schema version 1

- `schema_version`: manifest format version (`1`).
- `version` and `tag`: release version and source Git tag.
- `channel`: `stable` for normal version tags; `prerelease` for versions with a
  hyphen suffix such as `0.2.0-rc.1`. Prereleases do not replace GitHub's latest
  stable release URL.
- `stdlib`: standard-library-only archive metadata.
- `targets`: map keyed by Rust target triple. Each target has `binary` metadata
  for a binary-only archive and `bundle` metadata for a binary plus standard
  library archive.
- Each asset metadata object includes `filename`, absolute `url`, and lowercase
  SHA-256 digest in `sha256`. AlacoUp should verify the digest before extraction.

## Release archives

Each platform receives two `.tar.gz` archives:

- `alaco-VERSION-TARGET-binary.tar.gz`: executable and install notes, no stdlib.
- `alaco-VERSION-TARGET.tar.gz`: executable, `std/` modules, and install notes.

One platform-independent archive is published per release:

- `alaco-VERSION-stdlib.tar.gz`: `std/` modules and install notes, no executable.

Each archive contains `std/*.aco` under its extraction root. For a combined
archive, set `ALACO_STDLIB_DIR` to the extracted archive root. For a separate
stdlib install, set `ALACO_STDLIB_DIR` to the extracted stdlib archive root, or
copy that archive's `std/` directory into `~/.alaco/stdlib/std/`.

`SHA256SUMS` contains checksums for all three archive types and their per-target
variants. The release workflow validates that every target has both binary and
combined archives before publishing the manifest.
