# Target compatibility

Alaco currently has two output paths: native C++ compilation and JavaScript transpilation. They share the language front end, but they do not have identical runtime capabilities. Treat this page as a compatibility guide for the documented 0.2.0 release behavior, not as a promise of permanent parity.

## At a glance

| Feature area | Native C++ | JavaScript output | Notes |
| --- | --- | --- | --- |
| Parsing and semantic analysis | Shared front end | Shared front end | `check` validates Alaco source before either output path. |
| Functions, structs, conditions, loops, and `match` | Supported documented subset | Common constructs are handled | Test target-specific behavior in the runtime you will deploy. |
| Timed variables | Supported | Not supported | Timed variables use a native timer thread per variable. |
| Shell expressions | Supported | Not supported | Shell commands also depend on the host operating system and installed programs. |
| Filesystem, environment, arguments, paths, JSON, and TOML | Native runtime helpers | JavaScript runtime helpers | Individual APIs can still differ or be missing; check each module's notes. |
| HTTP | Platform-native implementation | Calls external `curl` synchronously | JavaScript requires `curl` on `PATH`. |
| HTTP build dependencies | Platform-specific | `curl` at runtime | Non-Windows Unix native builds require OpenSSL development headers/libraries for HTTP. |
| Executable output | Native executable | `.js` file | JavaScript output requires Node.js to run. |

The JavaScript backend handles common intermediate-representation constructs, but not every native runtime operation has a JavaScript implementation. A successful `transpile` command is not a guarantee that every feature will behave identically when executed.

## Native C++ requirements

Native builds generate C++ and invoke the host C++ toolchain. `clang++` is recommended on macOS and Linux. On Windows, use the supported MSVC environment described by the release installation notes. Run `alaco doctor` to check the compiler's native build prerequisites.

The native HTTP implementation is platform-specific:

- Windows uses WinHTTP.
- macOS uses CFNetwork.
- Other Unix-like systems use native sockets and OpenSSL 1.1.0 or newer for TLS. The OpenSSL development headers and libraries are needed when compiling an HTTP-using program.

HTTPS certificate and hostname verification are enabled by the native implementations.

## JavaScript requirements

Transpile a source file and run the result with Node.js:

```sh
alaco transpile src/main.aco --output target/main.js
node target/main.js
```

The JavaScript HTTP helpers use synchronous calls to the external `curl` executable because the current Alaco HTTP API is synchronous. Ensure `curl` is installed and available on `PATH` at runtime.

Timed variables and shell expressions are not supported by this backend. Some native standard-library operations may also be missing from JavaScript output. If a program depends on a particular module function, include a small target-specific test rather than assuming support from the module's existence.

## Choosing a target

Choose native C++ when you need timed variables, shell expressions, or platform-native executable output. Choose JavaScript when your program uses the supported subset and needs to run in a Node.js environment. If portability matters, keep platform-specific effects behind small functions and test both generated outputs.

## Compatibility test checklist

- [ ] Run `alaco check` on the source.
- [ ] Run `alaco run` to exercise the native path.
- [ ] Run `alaco transpile` and execute the generated file with the intended Node.js version.
- [ ] Exercise each standard-library function the program depends on.
- [ ] Test empty files, missing environment variables, failed HTTP requests, and other sentinel-return cases.
- [ ] Check that the program does not rely on timed variables or shell expressions when targeting JavaScript.

Related pages: [Compiler targets](/guide/targets), [Standard library](/reference/standard-library), and [Current limitations](/reference/limitations).
