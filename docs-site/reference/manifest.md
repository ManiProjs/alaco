# Project manifest

An Alaco project uses a file named `Alaco.toml` at its root. `alaco new` and `alaco init` create one for you.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2026"
entry = "src/main.aco"

[dependencies]
# Local package, resolved relative to this manifest.
local_tools = { path = "../local_tools", version = "^0.1.0" }
# Git package, resolved from a matching v-prefixed version tag.
colors = { git = "https://github.com/example/colors-alaco.git", version = "^1.2.0" }
```

## Package fields

| Field | Required | Meaning |
| --- | --- | --- |
| `package.name` | Yes | Package identifier. |
| `package.version` | Yes | Semantic version string. |
| `package.edition` | No | Currently supports `2026`. |
| `package.entry` | No | Relative `.aco` entry point; defaults to `src/main.aco`. |
| `dependencies` | No | Dependencies keyed by import name. Use a version string for an unsupported registry-style declaration, or an inline table with exactly one `path` or `git` source. |

The entry path must be relative and point to a `.aco` file inside the package. Run `alaco manifest` to validate and inspect the manifest. Use `alaco add` to add local or Git dependencies; Git dependencies are resolved to a commit and recorded in `Alaco.lock`.

## Current package boundaries

## Dependency source fields

| Field | Applies to | Meaning |
| --- | --- | --- |
| `path` | Local dependency | Path to a package directory, relative to the dependent project's manifest. |
| `git` | Git dependency | Git repository URL. |
| `version` | Either | Supported numeric semantic-version constraint. |
| `rev` | Git only | Git revision or commit to check out. |
| `tag` | Git only | Exact Git tag to check out. |
| `branch` | Git only | Remote branch to resolve to a commit. |

Exactly one of `path` and `git` must be set. At most one of `rev`, `tag`, and `branch` can be set. For automatic Git version selection, Alaco considers only `vMAJOR.MINOR.PATCH` tags and picks the highest matching numeric version. There is no central package registry.

See [Dependencies and packages](/guide/dependencies) for examples and [Lockfile reference](/reference/lockfile) for the resolved metadata format.
