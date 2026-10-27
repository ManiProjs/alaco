# Compiler targets

Alaco separates front-end analysis from target-specific code generation. The current compiler offers a native C++ path and a JavaScript transpiler. These targets have different runtime requirements and feature boundaries.

## Native C++

The native path is the primary executable workflow:

```sh
alaco check src/main.aco
alaco build src/main.aco
alaco run src/main.aco
```

The compiler validates the program, generates C++, then invokes the system C++ compiler. Native builds therefore require a supported C++ toolchain. The exact compiler and linker behavior depends on the host platform.

Use `alaco build --output path/to/program` to choose an output path. Project builds use the project's `target/` directory by default.

## JavaScript output

Use the `transpile` command to generate a JavaScript file:

```sh
alaco transpile examples/javascript.aco --output examples/javascript.js
node examples/javascript.js
```

`transpile` writes a sibling `.js` file by default. Use `--output <path>` to choose a different destination; there is currently no `--target` option because JavaScript is the transpiler's target.

The backend currently handles common IR constructs such as functions, structs, control flow, loops, match branches, interpolation, and console output. Its runtime includes helpers for filesystem, environment, command-line arguments, paths, JSON, a limited TOML subset, synchronous HTTP, and timing.

### Differences and limitations

- Timed variables and shell expressions are not supported by JavaScript output.
- Some native standard-library functions may have no JavaScript implementation yet.
- TOML support is a small scalar-oriented subset, not a complete TOML 1.0 implementation.
- `std.http` uses the external `curl` executable for JavaScript output. Install `curl` and ensure it is on `PATH` if a transpiled program makes HTTP requests.
- Successful transpilation is not a guarantee that every target runtime behavior matches native C++.

Keep a small executable test for each feature you depend on, and run the generated JavaScript file under the Node.js version you intend to support.

## Choosing a target

| Need | Suggested target |
| --- | --- |
| Native executable built on the current machine | C++ |
| Experimenting with JavaScript integration | JavaScript output |
| Platform-specific system access | Check the relevant standard-library implementation first |

The target matrix will grow as the compiler's shared IR and runtime abstractions mature.
