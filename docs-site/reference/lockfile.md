# Lockfile reference

`Alaco.lock` records the resolved dependency set for an Alaco project. The compiler writes it when dependencies are resolved during a build. Commit it alongside `Alaco.toml` for repeatable dependency selection across machines.

## Example

```toml
lock-version = 1

[[package]]
name = "colors"
version = "1.2.0"
source = "git:https://github.com/example/colors-alaco.git"
resolved = "target/.alaco/packages/colors-0123456789abcdef"
revision = "0123456789abcdef0123456789abcdef01234567"
```

The exact `resolved` path and revision depend on the project and repository. Treat the values above as an illustrative shape, not a usable dependency.

## Fields

| Field | Meaning |
| --- | --- |
| `lock-version` | Lockfile format version. |
| `package[].name` | Dependency name used by the project. |
| `package[].version` | Version read from the dependency's `Alaco.toml`. |
| `package[].source` | Dependency source identity (`git:` URL or `path:` declaration). |
| `package[].resolved` | Resolved checkout path or local package path. Relative paths are interpreted from the lockfile's directory. |
| `package[].revision` | Resolved Git commit ID for Git dependencies. Path dependencies are live and have no immutable Git revision. |

## Reproducibility and updates

When a Git dependency has a matching lock entry, Alaco uses the recorded commit rather than silently switching to a newer tag or a moving branch. This keeps ordinary builds repeatable. To intentionally change a resolved version, edit the dependency declaration or remove the relevant package entry from the lockfile, then run `alaco build` to resolve again. Inspect the new lockfile diff before committing it.

Do not hand-edit commit IDs or resolved checkout paths. If the file is malformed, Alaco reports a lockfile error rather than silently treating it as empty. `alaco clean` removes cached Git checkouts; the lockfile itself remains in the project root.

See [Dependencies and packages](/guide/dependencies) and the [manifest reference](/reference/manifest).
