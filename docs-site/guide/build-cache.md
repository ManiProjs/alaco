# Build artifacts and troubleshooting

The current CLI does not expose incremental-build inspection flags such as `--explain` or `--no-cache`. Treat generated files under `target/` as disposable artifacts and keep source files, `Alaco.toml`, and `Alaco.lock` as the reproducible inputs to your project.

## Clear generated state

```sh
alaco clean
```

This removes the current project's generated build artifacts under `target/`. It does not remove source files. If your project uses Git dependencies cached under `target/`, a later build may need network access to resolve them again.

## A clean troubleshooting cycle

When a native executable behaves unexpectedly:

1. Run `alaco check src/main.aco` to confirm that the source still passes parsing and semantic analysis.
2. Run `alaco clean` to remove generated artifacts.
3. Run `alaco build src/main.aco --output target/app` to create a fresh executable.
4. Run the exact executable and reproduce the behavior with a small, known input.
5. Run `alaco test` and inspect test output; printed messages alone are not an automatic assertion framework.

For the normal edit-check-run loop, see [project workflow](/guide/project-workflow). For command options, see the [CLI reference](/reference/cli).
