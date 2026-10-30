# Standard library reference

Alaco's standard library is written in Alaco and backed by small native runtime helpers where operating-system access is needed. `std.io` is available by default. Import other modules explicitly, for example `import std.math;`, then call their functions through the module name (`math.abs(-4)`).

For the exact declarations and parameter/return types from the bundled standard-library sources, see [Standard-library signatures](/reference/standard-library-signatures). This page focuses on behavior and caveats.

## Conventions and error handling

- Standard modules are imported with `import std.<module>;` and normally accessed through the final module name, such as `fs.exists(path)`.
- Functions use the return types documented below. APIs that return `Bool` or a sentinel value should be checked when failure matters.
- Several APIs intentionally use simple fallbacks instead of rich error values. An empty string may mean either valid empty data or failure; see each module's notes.
- Filesystem paths generally use the process's current working directory as their base unless the path is absolute.
- This is a compact standard library, not a full replacement for the platform APIs or mature JSON/TOML/HTTP libraries.

## `std.io` — output

Available automatically; no import is required.

| Function | Description |
| --- | --- |
| `print(value)` | Write a value to standard output without a newline. |
| `println(value)` | Write a value to standard output followed by a newline. |
| `eprint(value)` | Write a value to standard error without a newline. |
| `eprintln(value)` | Write a value to standard error followed by a newline. |

Printing supports `String`, `Int`, `Float`, `Bool`, and values with a generated runtime formatter, such as supported structs.

Use `print` when output should continue on the same line and `println` when a trailing newline is useful. The `eprint` variants write to standard error and are suitable for diagnostic messages.

## `std.math` — integer helpers

Import with `import std.math;`.

| Function | Behavior |
| --- | --- |
| `min(a, b)` / `max(a, b)` | Return the smaller / larger integer. |
| `abs(value)` | Return the non-negative magnitude of an integer. |
| `square(value)` / `cube(value)` | Return the value multiplied by itself two / three times. |
| `sign(value)` | Return `-1`, `0`, or `1`. |
| `clamp(value, lower, upper)` | Limit a value to the inclusive bounds. |
| `in_range(value, lower, upper)` | Test whether the value is within the inclusive bounds. |
| `is_even(value)` / `is_odd(value)` | Test integer parity. |
| `gcd(left, right)` | Greatest common divisor using the Euclidean algorithm. |
| `lcm(left, right)` | Least common multiple; returns zero if either input is zero. |
| `pow_int(base, exponent)` | Integer exponentiation; non-positive exponents return `1`. |
| `is_prime(value)` | Test primality by trial division up to the square root. |
| `factorial(value)` | Factorial for 0 through 20; returns `-1` for negative or too-large inputs. |
| `ceil_div(value, divisor)` | Integer division rounded upward; returns `0` for a zero divisor. |
| `triangular(value)` | Sum from 1 through `value`; returns `-1` for negative or out-of-range inputs. |
| `is_power_of_two(value)` | True for positive powers of two. |
| `is_multiple_of(value, divisor)` | Test divisibility; returns false when the divisor is zero. |
| `round_up_to_multiple(value, step)` | Round upward to a multiple; non-positive steps leave the value unchanged. |

## `std.string` — string helpers

Import with `import std.string;`.

| Function | Behavior |
| --- | --- |
| `concat(left, right)` | Concatenate two strings. The `+` operator also concatenates strings. |
| `is_empty(value)` | Return whether a string is empty. |
| `repeat_string(value, count)` | Repeat a string `count` times; a non-positive count performs no iterations. |
| `surround(value, wrapper)` | Add the wrapper string before and after a value. |
| `join3(first, separator, second, third)` | Join three strings with the same separator. |
| `is_not_empty(value)` | Inverse of `is_empty`; whitespace still counts as content. |
| `concat3(first, second, third)` | Concatenate three strings. |
| `join4(first, separator, second, third, fourth)` | Join four strings with a shared separator. |
| `prefix_suffix(value, prefix, suffix)` | Add different strings before and after a value. |

## `std.geometry` — geometry helpers

Import with `import std.geometry;`.

| Function | Behavior |
| --- | --- |
| `rectangle_area(width, height)` | Multiply width by height. |
| `rectangle_perimeter(width, height)` | Calculate `2 * (width + height)`. |
| `squared_distance(x1, y1, x2, y2)` | Return squared Euclidean distance (no square root). |
| `manhattan_distance(x1, y1, x2, y2)` | Return Manhattan distance. |
| `is_inside_rectangle(x, y, left, top, right, bottom)` | Check whether a point is within the inclusive rectangle bounds. |
| `absolute(value)` | Return the non-negative magnitude of an integer. |
| `rectangle_diagonal_squared(width, height)` | Return the squared diagonal length without calculating a square root. |
| `triangle_area(base, height)` | Calculate `(base * height) / 2` using integer arithmetic. |
| `circle_area(radius)` | Calculate π × radius² using `Float`. |
| `circle_circumference(radius)` | Calculate 2 × π × radius using `Float`. |

## `std.stats` — small integer statistics

Import with `import std.stats;`.

| Function | Behavior |
| --- | --- |
| `sum3(a, b, c)` | Sum three integers. |
| `average2(a, b)` / `average3(a, b, c)` | Calculate an integer average; fractional parts are discarded by integer division. |
| `min2(a, b)` / `max2(a, b)` | Return the minimum / maximum of two integers. |
| `min3(a, b, c)` / `max3(a, b, c)` | Return the minimum / maximum of three integers. |
| `median3(a, b, c)` | Return the middle of three integers without sorting. |
| `sum4(a, b, c, d)` / `average4(a, b, c, d)` | Sum or integer-average four values. |
| `percentage(value, total)` | Return the integer percentage; returns `0` when total is zero. |
| `sum2(a, b)` | Sum two integers. |
| `sum5(a, b, c, d, e)` / `average5(a, b, c, d, e)` | Sum or integer-average five integers. |
| `range3(a, b, c)` | Difference between the maximum and minimum of three values. |

## `std.units` — unit conversions

Import with `import std.units;`. Most conversions accept and return `Float`. `bytes_to_kibibytes` and `kibibytes_to_bytes` use `Int` and therefore discard fractional KiB through integer division.

| Function | Conversion |
| --- | --- |
| `celsius_to_fahrenheit(value)` | Celsius → Fahrenheit |
| `fahrenheit_to_celsius(value)` | Fahrenheit → Celsius |
| `kilometers_to_miles(value)` | Kilometers → miles |
| `miles_to_kilometers(value)` | Miles → kilometers |
| `kilograms_to_pounds(value)` | Kilograms → pounds |
| `pounds_to_kilograms(value)` | Pounds → kilograms |
| `meters_to_feet(value)` / `feet_to_meters(value)` | Meters ↔ feet |
| `liters_to_us_gallons(value)` / `us_gallons_to_liters(value)` | Liters ↔ US gallons |
| `meters_to_kilometers(value)` / `kilometers_to_meters(value)` | Meters ↔ kilometers |
| `centimeters_to_inches(value)` / `inches_to_centimeters(value)` | Centimeters ↔ inches |
| `seconds_to_minutes(value)` / `minutes_to_seconds(value)` | Seconds ↔ minutes |
| `hours_to_seconds(value)` | Hours → seconds |
| `bytes_to_kibibytes(value)` / `kibibytes_to_bytes(value)` | Bytes ↔ KiB, using 1024 bytes per KiB; integer division truncates. |

## `std.validation` — simple predicates

Import with `import std.validation;`.

| Function | Behavior |
| --- | --- |
| `is_non_empty(value: String)` | True when the string is not empty. Whitespace-only strings count as non-empty. |
| `is_in_range(value, lower, upper)` | True when an integer lies within inclusive bounds. |
| `is_positive(value)` | True when an integer is greater than zero. |
| `is_non_negative(value)` | True when an integer is zero or greater. |
| `is_negative(value)` / `is_zero(value)` / `is_non_zero(value)` | Sign and zero checks. |
| `is_valid_port(value)` | True for TCP/UDP port numbers from 1 through 65535. |
| `is_valid_percentage(value)` | True for an integer from 0 through 100. |
| `is_valid_year(value)` | True for a year number from 1 through 9999. |
| `is_valid_ipv4_octet(value)` | True for an IPv4 address component from 0 through 255. |

## `std.fs` — filesystem operations

Import with `import std.fs;`. Paths are interpreted relative to the process's current working directory unless absolute. Operations return `Bool` where applicable so callers can check success.

| Function | Return | Behavior and limitations |
| --- | --- | --- |
| `read_text(path: String)` | `String` | Read file contents as text. A failed read returns `""`, so failure cannot be distinguished from a successfully read empty file using this function alone. |
| `write_text(path: String, contents: String)` | `Bool` | Write/truncate a file and report success. |
| `file_size(path: String)` | `Int` | Return file size in bytes, or `-1` if the size cannot be read. |
| `copy(source: String, destination: String)` | `Bool` | Copy a file, replacing an existing destination. Returns false if the copy fails. |
| `exists(path: String)` | `Bool` | Check whether the path exists. |
| `remove(path: String)` | `Bool` | Remove a file or an empty directory; reports failure as false. |
| `create_dir(path: String)` | `Bool` | Create a directory and any missing parent directories; reports failure as false. |

`file_size` uses bytes, not characters. File operations can fail because of permissions, invalid paths, missing files, or other operating-system errors.

For operations that return `Bool`, treat `false` as failure rather than assuming the filesystem was changed. `write_text` overwrites the destination contents; `copy` may replace an existing destination. Avoid using destructive operations on paths you have not validated.

## `std.env` — process environment

Import with `import std.env;`.

| Function | Return | Behavior |
| --- | --- | --- |
| `get(name: String)` | `String` | Read an environment variable; returns `""` if it is missing. |
| `has(name: String)` | `Bool` | Test whether the variable exists, including when its value is empty. |
| `set(name: String, value: String)` | `Bool` | Set an environment variable for the current process. |
| `current_dir()` | `String` | Return the current working directory, or an empty string if it cannot be obtained. |

Use `has` together with `get` when a missing variable must be distinguished from a variable set to an empty string. Environment changes affect the running process and are not a way to modify the parent shell's environment.

## `std.args` — command-line arguments

Import with `import std.args;`. Argument indices are zero-based and exclude the executable name. This module provides raw arguments as well as helpers for positional values and simple flags/options.

| Function | Behavior |
| --- | --- |
| `count()` | Number of raw arguments, including flags, option values, and positional arguments. |
| `get(index: Int)` | Return the raw argument at an index, or `""` when out of range. |
| `positional_count()` | Number of positional arguments after recognized options are skipped. |
| `positional(index: Int)` | Return a positional argument, or `""` when out of range. |
| `has_flag(name: String)` | Check whether an exact flag token is present. |
| `has_option(name: String)` | Check whether an option appears as `--name` or `--name=value`. |
| `option(name: String, fallback: String)` | Read an option value in either `--name value` or `--name=value` form, or return the fallback. |

The parser is intentionally lightweight: it does not validate a command-line schema or generate help text. Calling `option` registers that value-taking option with the positional-argument helper. Call it before relying on `positional_count()` or `positional(index)`, so its value is not mistaken for a positional argument.

## `std.path` — lexical path helpers

Import with `import std.path;`. These functions inspect or combine path strings; they do not require the referenced file to exist.

| Function | Behavior |
| --- | --- |
| `join(left: String, right: String)` | Join two path components using the host platform's path rules. |
| `basename(value: String)` | Return the final path component. |
| `dirname(value: String)` | Return the parent path component. |
| `extension(value: String)` | Return the final extension without the dot (for `archive.tar.gz`, returns `gz`). |
| `stem(value: String)` | Return the final component without its final extension (for `archive.tar.gz`, returns `archive.tar`). |
| `is_absolute(value: String)` | Check whether the path is absolute according to the host platform. |

Path parsing and joining follow `std::filesystem` semantics, so separator and absolute-path behavior can differ between operating systems.

## `std.json` — JSON text utilities

Import with `import std.json;`.

| Function | Behavior and limitations |
| --- | --- |
| `is_valid(text: String)` | Validate a JSON value, including nested arrays and objects, strings, numbers, booleans, and `null`. |
| `escape(text: String)` | Escape string contents for use inside a JSON string; it does not add surrounding quotes. |
| `quote(text: String)` | Escape a string and add JSON double quotes. |
| `has_key(text: String, key: String)` | Check for a key in a top-level JSON object. |
| `is_null(text: String, key: String)` | Return true only when the top-level key exists and its value is JSON `null`. |
| `get_string(text: String, key: String, fallback: String)` | Read a top-level string property or return the fallback. |
| `get_int(text: String, key: String, fallback: Int)` | Read a top-level integer property or return the fallback. |
| `get_bool(text: String, key: String, fallback: Bool)` | Read a top-level boolean property or return the fallback. |

These typed accessors read top-level object properties; they do not expose a dynamic JSON value/map/list type or recursively navigate nested objects. String escape decoding covers common escapes.

The helpers are intended for small configuration payloads and straightforward lookups. If a key is nested, or if you need to preserve arbitrary JSON values and types, these accessors are not enough by themselves. `quote` is useful when constructing a JSON string literal; `escape` only escapes the contents and does not add quotes.

## `std.toml` — TOML configuration utilities

Import with `import std.toml;`.

| Function | Behavior and limitations |
| --- | --- |
| `is_valid(text: String)` | Validate a limited subset of scalar TOML assignments and section headers. |
| `has_key(text: String, key: String)` | Check for a top-level key. |
| `get_string(text: String, key: String, fallback: String)` | Read a top-level quoted string or return the fallback. |
| `get_int(text: String, key: String, fallback: Int)` | Read a top-level integer or return the fallback. |
| `get_bool(text: String, key: String, fallback: Bool)` | Read a top-level boolean or return the fallback. |

This is not a complete TOML 1.0 parser. Accessors currently target top-level scalar keys; table-scoped keys, full arrays/inline tables, dates, and every grammar edge case are not fully supported.

Treat this module as a convenience for simple scalar settings rather than as a general-purpose TOML implementation. A document accepted by this subset validator should not be taken as proof that every TOML consumer will interpret it identically.

## `std.http` — HTTP requests

Import with `import std.http;`.

| Function | Behavior and limitations |
| --- | --- |
| `is_url(url: String)` | Check whether a URL starts with `http://` or `https://`. |
| `get(url: String)` | Return the response body for a successful GET request. |
| `post_json(url: String, body: String)` | POST a body with `Content-Type: application/json` and return the response body. |
| `response(url: String)` | Perform a GET and return a JSON object containing `status`, `body`, `success`, and `error`. |
| `post_json_response(url: String, body: String)` | Perform a JSON POST and return the same structured response fields. |
| `status(url: String)` | Perform a GET and return its HTTP status; `0` means no HTTP response was obtained. |
| `is_success(url: String)` | Perform a GET and return whether its status is in the 2xx range. |
| `error(url: String)` | Perform a GET and return its transport/request error text. |

For native builds, no `curl` executable is spawned: Windows uses WinHTTP, macOS uses CFNetwork, and other Unix-like builds use OpenSSL 1.1.0+ for TLS with native sockets for transport. The OpenSSL development headers and libraries must be installed when compiling an HTTP-using program on those Unix-like systems. HTTPS certificate and hostname verification are enabled. For JavaScript output, the synchronous HTTP helpers invoke the external `curl` executable because Node.js networking APIs are asynchronous. Ensure `curl` is installed and on `PATH` when running transpiled programs that use HTTP.

The body-only `get` and `post_json` helpers return an empty string for transport failures and non-2xx responses, so an empty body is ambiguous. The structured `response` and `post_json_response` variants include status, body, success, and error fields in JSON. `status`, `is_success`, and `error` each issue their own GET request; do not call all three when one `response` call would provide the information from a single request. The API does not expose response headers.

Use `is_url` only as a lightweight scheme check, not as complete URL validation. Do not send secrets to untrusted endpoints. Prefer the structured response helpers when the caller needs to distinguish a successful empty response from a failed request or inspect the HTTP status.

Runnable examples: `examples/http_demo.aco`, `examples/json_demo.aco`, `examples/toml_demo.aco`, and `examples/type_safety.aco`.

## Choosing an API

| Need | Module | Important limitation |
| --- | --- | --- |
| Print output | `std.io` (automatic) | Small set of printable value types |
| Integer math and predicates | `std.math`, `std.validation`, `std.stats` | Integer-focused; averages discard fractional parts |
| String composition | `std.string` | Basic helpers; `+` also concatenates strings |
| File and directory operations | `std.fs` | Some failures use `false`, `-1`, or `""` sentinels |
| Environment variables and current directory | `std.env` | Changes affect only the running process |
| Command-line arguments and options | `std.args` | Lightweight parsing; no schema validation or generated help |
| Manipulate path strings | `std.path` | Host platform path semantics apply |
| Validate/read simple JSON | `std.json` | Typed accessors read top-level keys only |
| Read simple TOML settings | `std.toml` | Supports a limited subset, not full TOML 1.0 |
| HTTP GET / JSON POST | `std.http` | Structured responses include status and error but not response headers |
| Timestamp and sleep | `std.timing` | Whole-second timestamp and millisecond sleep |

## `std.timing` — basic time utilities

Import with `import std.timing;`.

| Function | Behavior |
| --- | --- |
| `unix_seconds()` | Current Unix timestamp in whole seconds. |
| `sleep_ms(milliseconds: Int)` | Pause the current thread for the requested number of milliseconds; non-positive values do nothing. |

## Example

```text
import std.math;
import std.string;
import std.fs;

fn main() {
    println(math.clamp(12, 0, 10));
    println(string.repeat_string("ha", 3));

    if fs.write_text("hello.txt", "Hello, Alaco!") {
        println(fs.file_size("hello.txt"));
    }
}
```

The example creates `hello.txt` in the program's working directory. Remove it if you do not want to keep the file.
