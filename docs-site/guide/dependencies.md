# Dependencies and packages

Alaco supports local path dependencies and Git-hosted packages. There is no central package registry, so every dependency must name a local directory or a Git repository.

## Declare a dependency

The current CLI does not provide an `alaco add` command. Edit the `[dependencies]` table in `Alaco.toml` directly, then build the project to resolve the declared dependency and update `Alaco.lock`.

### Local package

Given this layout:

```text
workspace/
  app/
    Alaco.toml
    src/main.aco
  shared/
    Alaco.toml
    src/lib.aco
```

Declare the local package in `app/Alaco.toml`:

```sh
cd app
```

Add this entry under `[dependencies]` in `app/Alaco.toml`:

```toml
shared = { path = "../shared", version = "^0.1.0" }
```

Path dependencies are live: edits in `shared/` are used by the next build. The dependency directory must contain its own valid `Alaco.toml`.

### Git package

Add a Git dependency under `[dependencies]` in `Alaco.toml`:

```toml
utility = { git = "https://github.com/example/utility-alaco.git", version = "^1.2.0" }
```

Alaco fetches the repository, selects a matching version tag, checks out the selected commit, and records it in `Alaco.lock`. Automatic version selection recognizes semantic-version tags prefixed with `v`, such as `v1.2.0` and `v1.10.0`. Bare tags like `1.10.0` and tags like `release-1.10.0` are ignored by automatic selection. Versions are compared numerically.

You can instead select a specific Git source:

```toml
utility_tag = { git = "https://github.com/example/utility-alaco.git", tag = "v1.2.0" }
utility_rev = { git = "https://github.com/example/utility-alaco.git", rev = "0123456789abcdef" }
utility_branch = { git = "https://github.com/example/utility-alaco.git", branch = "main" }
```

`--tag`, `--rev`, and `--branch` are mutually exclusive. A branch is resolved to a commit and recorded in the lockfile; it is not automatically followed on every build.

Use `--name` to choose the dependency's import name if the inferred name is unsuitable:

The dependency key in the manifest is the import name, so use `utility` as the key when the source should be imported as `utility`.

## Version constraints

For Git dependencies, the supported constraints are:

| Constraint | Meaning |
| --- | --- |
| `"*"` | Any supported three-part numeric version. |
| `"=1.2.3"` | Exactly `1.2.3`. |
| `"^1.2.3"` | Compatible versions in the same major version when the major is nonzero. |
| `"~1.2.3"` | Versions in the same major and minor version, at least `1.2.3`. |
| `">=1.2.3"` | `1.2.3` or newer. |

Two-part constraints such as `"^1.2"` are also accepted and treated as `1.2.0`. The current resolver supports numeric `major.minor.patch` tags; it does not implement full SemVer prerelease ordering.

## Import a dependency

Packages expose source modules from their `src/` directory. The package root library is `src/lib.aco`:

```aco
import utility;
import utility.format;

fn main() {
    println(utility.greet("Alaco"));
}
```

This example assumes `src/lib.aco` provides `greet`. The dotted import makes the module in `src/format.aco` available under the `format` namespace; call its functions through that namespace. Dotted import paths map to nested files under `src/`.

## Lockfile behavior

`Alaco.lock` records resolved dependency metadata and immutable Git commit IDs. Commit the lockfile with your application so builds can reuse the same Git revisions. Once a Git dependency is locked, builds honor that revision even if a newer matching tag appears upstream. To intentionally move to a newer version, update the dependency declaration or remove its existing lock entry and run `alaco build` to resolve it again; review the resulting diff before committing.

Git checkouts are cached under `target/.alaco/packages/`. `alaco clean` removes the project-local build and dependency cache, so a later build may need network access to fetch Git dependencies again. Local path dependencies are not copied into that cache.

## Troubleshooting

- **No matching Git tag:** check the version constraint and ensure the repository has a matching `vMAJOR.MINOR.PATCH` tag.
- **Git fetch or clone fails:** verify the URL, network access, and your Git credentials.
- **Dependency import does not exist:** check that the package contains `src/lib.aco` for `import name;`, or the expected nested `.aco` file for a dotted import.
- **A path dependency cannot be resolved:** check the path relative to the project's `Alaco.toml`, not your shell's current directory.
- **A lockfile is invalid:** inspect `Alaco.lock` for hand-edited or malformed TOML; do not manually invent commit IDs.

See the [manifest reference](/reference/manifest), [CLI reference](/reference/cli), and [current limitations](/reference/limitations) for related details.
