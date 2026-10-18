# Incremental builds

`alaco build` caches successful native builds so an unchanged project can skip C++ generation and compilation. The cache is an optimization; source files and the compiler remain the source of truth.

## How cache validation works

Alaco tracks the transitive source inputs for the entry point, including imported local modules, dependency modules, and automatically loaded standard-library modules. It compares their fingerprints along with the Alaco compiler and native C++ toolchain identity. It also fingerprints the output executable so a missing or externally modified artifact is not accepted as a valid cache hit.

The cache state is stored under `target/.alaco/cache/`. Keep `target/` as generated output rather than source control; it can be recreated.

## Explain a build decision

Use `--explain` to see why Alaco is rebuilding or reusing the executable:

```sh
alaco build --explain
```

You can pass a source file or an output path as usual:

```sh
alaco build src/main.aco --explain
alaco build src/main.aco --output target/my-app --explain
```

Depending on the state of the project, the explanation can report that no cache state exists, list changed source inputs, identify a missing or changed output artifact, or confirm that tracked inputs and compiler identity match.

## Force a clean compilation

Use `--no-cache` to ignore the cached build and compile again:

```sh
alaco build --no-cache
```

This is useful when diagnosing a toolchain problem or confirming that a successful build does not depend on stale generated output. It does not delete the cache; a successful forced build writes fresh cache state afterward.

## Clear generated state

```sh
alaco clean
```

This removes the current project's generated build artifacts and local Git dependency checkout cache. It does not remove source files or local path dependencies. If the project uses Git dependencies, the next resolution may need to fetch them again.

## What incremental builds do not do

- They do not skip semantic checking when you run `alaco check`.
- They do not make a stale executable safe to run after its inputs change; `build` checks the graph before reusing the output.
- They do not replace tests. Run `alaco test` after changing behavior, and use `alaco build --no-cache` when you need to rule out cache reuse during troubleshooting.

For the normal edit-check-run loop, see [project workflow](/guide/project-workflow). For command options, see the [CLI reference](/reference/cli).
