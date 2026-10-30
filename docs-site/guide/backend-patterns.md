# Practical application patterns

This page combines Alaco's current standard-library modules into small backend-oriented patterns. The library intentionally favors simple return values over a large error-handling framework, so check sentinel values and understand the limitations of each API.

## Read configuration safely

For a small scalar configuration file, `std.fs` can read the file and `std.toml` can validate and retrieve selected top-level values.

```aco
import std.fs;
import std.toml;

fn main() {
    let text = fs.read_text("settings.toml");

    if text == "" {
        eprintln("Could not read settings.toml, or the file is empty");
    } else if !toml.is_valid(text) {
        eprintln("Settings are not valid TOML for Alaco's supported subset");
    } else {
        let port = toml.get_int(text, "port", 8080);
        println("Configured port: ${port}");
    }
}
```

`read_text` returns an empty string on read failure, so this example cannot distinguish a missing file from a valid empty file. `std.toml` is a limited scalar-oriented subset, not a full TOML 1.0 parser; it is best suited to simple top-level settings.

## Read environment variables

Use `std.env` for process configuration. Call `has` if a missing variable must be distinguished from one that exists with an empty value.

```aco
import std.env;

fn main() {
    if env.has("APP_MODE") {
        println("Mode: " + env.get("APP_MODE"));
    } else {
        println("Mode: development");
    }
}
```

Environment changes affect only the running process; they cannot modify the parent shell. Avoid printing secrets such as API tokens into logs.

## Parse command-line options

`std.args` offers simple helpers for flags, options, and positional values. An option value may be supplied as `--name value` or `--name=value`.

```aco
import std.args;

fn main() {
    let output = args.option("--output", "result.txt");

    if args.has_flag("--verbose") {
        println("Verbose mode enabled");
    }

    println("Output path: " + output);
}
```

The argument parser is deliberately lightweight: it does not validate a command schema or generate help text. Call `option` for value-taking options before relying on `positional_count()` and `positional(index)`, so recognized option values are not mistaken for positional arguments. See the [`std.args` reference](/reference/standard-library#std-args-command-line-arguments) for the full API.

## Validate JSON before reading fields

`std.json` validates a JSON value and provides typed accessors for top-level object properties.

```aco
import std.json;

fn main() {
    let payload = "{\"name\":\"Alaco\",\"enabled\":true}";

    if json.is_valid(payload) {
        let name = json.get_string(payload, "name", "unknown");
        let enabled = json.get_bool(payload, "enabled", false);
        println("Service: ${name}");
        println("Enabled: ${enabled}");
    } else {
        eprintln("Invalid JSON input");
    }
}
```

These accessors only read top-level typed fields. They do not provide a dynamic JSON tree or navigate nested objects. When constructing JSON manually, use `json.quote` to escape a string and add its surrounding quotes; `json.escape` only escapes the contents.

## Use HTTP with explicit failure expectations

For a quick body-only request, `http.get` returns the body on success and an empty string for transport failures and non-2xx responses. When the status matters, use `http.response` to make one request and receive a JSON object with `status`, `body`, `success`, and `error` fields.

```aco
import std.http;
import std.json;

fn main() {
    let result = http.response("https://example.com/health");
    if json.is_valid(result) {
        let status = json.get_int(result, "status", 0);
        let success = json.get_bool(result, "success", false);
        let error = json.get_string(result, "error", "");
        if success {
            println("HTTP status: ${status}");
        } else {
            eprintln("Request failed (${status}): " + error);
        }
    }
}
```

`is_url` only checks the URL scheme prefix; it is not full URL validation. The structured response does not expose response headers. The convenience functions `status`, `is_success`, and `error` each perform their own GET request, so prefer `response` when you need several pieces of information about one request.

Native HTTP uses platform-specific implementations. Transpiled JavaScript uses the external `curl` executable for synchronous HTTP calls, so install `curl` and ensure it is on `PATH` when using `std.http` from generated JavaScript. Do not send credentials to endpoints you do not trust.

## Work with files and paths

`std.path` manipulates path strings using host-platform path rules; `std.fs` performs the actual filesystem operation.

```aco
import std.fs;
import std.path;

fn main() {
    let directory = "data";
    let filename = path.join(directory, "summary.txt");

    if fs.create_dir(directory) {
        if fs.write_text(filename, "Generated by Alaco\n") {
            println("Wrote " + filename);
        } else {
            eprintln("Could not write output file");
        }
    } else {
        eprintln("Could not create output directory");
    }
}
```

Relative paths are based on the process's current working directory, not automatically on the `.aco` source file's directory. `write_text` overwrites an existing destination. Avoid destructive filesystem operations on paths you have not checked.

## Error-handling checklist

- Check `Bool` results from writes, copies, directory creation, and environment updates.
- Treat `-1` from `fs.file_size` as failure.
- Remember that `fs.read_text`, `env.get`, JSON/TOML fallback accessors, and HTTP helpers use empty strings or caller-provided fallbacks in cases where information can be lost.
- Keep network calls and filesystem writes out of code paths that should be deterministic and side-effect-free.
- Run `alaco check` first, then exercise the actual runtime path with `alaco run` or a focused test.

See the [standard-library reference](/reference/standard-library) for signatures and per-module limitations, and [Target compatibility](/reference/compatibility) for differences between native and JavaScript execution.
