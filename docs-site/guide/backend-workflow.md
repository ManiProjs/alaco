# Backend project workflow

Alaco can compile programs into native executables and provides basic filesystem, environment, JSON, TOML, command-line, and HTTP client helpers. This guide shows how to structure and validate a small backend-oriented utility without assuming features that are not part of the current runtime.

## Start with a project, not a loose file

Create a project scaffold and enter its directory:

```sh
alaco new status-tool
cd status-tool
```

The project manifest is `Alaco.toml`, and the default entry point is `src/main.aco`. Keep application code in `src/`, tests in `tests/`, and generated build output under `target/`. This makes the entry point, tests, and generated artifacts easier to reason about as the program grows.

```text
status-tool/
  Alaco.toml
  src/
    main.aco
    config.aco
  tests/
    config_test.aco
```

This is a suggested layout, not a requirement that every project contain every file. For manifest fields and dependency resolution, see [Projects & runtime](/guide/language-projects), [Dependencies & packages](/guide/dependencies), and the [manifest reference](/reference/manifest).

## Separate parsing from application decisions

Keep boundary work—reading environment variables, files, command-line options, or HTTP responses—separate from calculations that can operate on ordinary values. This helps make the central logic easier to test and prevents external effects from being repeated accidentally.

For example, a port number should be validated before it is used as configuration:

```aco
import std.validation;

fn choose_port(candidate: Int) -> Int {
    if validation.is_valid_port(candidate) {
        return candidate;
    } else {
        return 8080;
    }
}

fn main() {
    println("Selected port: ${choose_port(4040)}");
}
```

The fallback is application policy, not a claim that port `8080` is available. If configuration must distinguish a missing value from an invalid one, represent those cases explicitly in your control flow instead of silently applying a fallback.

## Choose the right target

Use the native target when you need a compiled executable and have the required C++ toolchain installed:

```sh
alaco check src/main.aco
alaco run src/main.aco
alaco test
alaco build src/main.aco --output target/status-tool
```

`check` catches parsing and semantic errors without building the executable. `run` compiles and runs a temporary native program. `test` runs `.aco` test programs, and `build` produces a reusable executable at the requested path.

JavaScript output is useful when integrating a supported Alaco program with a Node.js environment:

```sh
alaco transpile src/main.aco --output target/status-tool.js
node target/status-tool.js
```

Do not assume native and JavaScript behavior is identical. Read [Compiler targets](/guide/targets) and [Current limitations](/reference/limitations), then execute the generated output on the runtime you plan to deploy.

## Treat external effects as failure-prone

Several standard-library functions use simple sentinel values rather than structured errors. For example, `fs.read_text` returns an empty string when a read fails and `fs.file_size` returns `-1` when size lookup fails. The body-only HTTP helpers return an empty string for failed requests and non-success status codes; use `http.response` or `http.post_json_response` when you need status and error information from the same request. Check what each function guarantees before deciding whether a result is valid.

For configuration files:

1. Check whether the file exists when missing and empty files need different handling.
2. Read the contents and validate the supported JSON or TOML format.
3. Read each value using a typed accessor and an explicit fallback.
4. Validate application-level constraints such as port ranges before using the value.

The JSON and TOML helpers only expose a limited set of typed lookups. They are not substitutes for a dynamic JSON tree or a complete TOML implementation. See the [standard-library reference](/reference/standard-library) for the exact limits.

## Keep secrets out of source and logs

- Read deployment-specific values from the process environment rather than hard-coding credentials in `.aco` files.
- Use `env.has(name)` when a missing variable must be distinguished from a present-but-empty value.
- Never print access tokens, passwords, or complete authorization headers as debugging output.
- Avoid forwarding secrets to an HTTP endpoint unless you have verified that endpoint and its transport.
- Remember that environment changes made by `std.env.set` affect only the running process; they do not change the parent shell.

These are application-security practices, not language-enforced guarantees. Review logs and test fixtures before sharing them.

## Be clear about the server boundary

The current `std.http` API is an HTTP **client** with `get`, JSON `post_json`, and structured response helpers. It does not expose a listener/server API, routing framework, response headers, or an asynchronous request model. A native executable can be useful for backend utilities and client-side integrations, but do not interpret the HTTP client as a complete web-server framework.

For JavaScript output, HTTP helpers invoke the external `curl` executable synchronously, so `curl` must be available on `PATH`. Native HTTP uses platform-specific implementations and may have additional build dependencies; see the HTTP section of the [standard-library reference](/reference/standard-library#std-http-http-requests).

## Validate before shipping

Use a short, repeatable release checklist:

- [ ] Run `alaco check` against the project entry point.
- [ ] Run focused `.aco` tests and inspect their output; printed `PASS` text alone is not an automatic assertion.
- [ ] Build the exact target you intend to deploy.
- [ ] Run the resulting executable in a clean working directory with representative configuration and environment variables.
- [ ] Test missing files, invalid configuration, unavailable network endpoints, and other expected failure cases.
- [ ] Confirm that logs contain no secrets and that filesystem writes stay within intended paths.
- [ ] Keep `Alaco.toml` and `Alaco.lock` under version control when the project uses dependencies.

For detailed command syntax, see the [CLI reference](/reference/cli). For diagnostics, see [Diagnostics & troubleshooting](/guide/diagnostics).
