# Standard-library signatures

This page lists the function declarations in the standard-library sources bundled with the Alaco 0.2.0 release payload. It complements the [standard-library reference](/reference/standard-library), which explains behavior, return conventions, and platform caveats.

A declaration without an explicit `-> Type` has no return value. These are the library declarations, not a promise that every function is implemented identically by every output backend; see [target compatibility](/reference/compatibility).

## `std.args`

```aco
fn count() -> Int
fn get(index: Int) -> String
fn positional_count() -> Int
fn positional(index: Int) -> String
fn has_flag(name: String) -> Bool
fn has_option(name: String) -> Bool
fn option(name: String, fallback: String) -> String
```

## `std.env`

```aco
fn get(name: String) -> String
fn has(name: String) -> Bool
fn set(name: String, value: String) -> Bool
fn current_dir() -> String
```

## `std.fs`

```aco
fn read_text(path: String) -> String
fn file_size(path: String) -> Int
fn copy(source: String, destination: String) -> Bool
fn write_text(path: String, contents: String) -> Bool
fn exists(path: String) -> Bool
fn remove(path: String) -> Bool
fn create_dir(path: String) -> Bool
```

## `std.geometry`

```aco
fn rectangle_area(width: Int, height: Int) -> Int
fn rectangle_perimeter(width: Int, height: Int) -> Int
fn squared_distance(x1: Int, y1: Int, x2: Int, y2: Int) -> Int
fn absolute(value: Int) -> Int
fn manhattan_distance(x1: Int, y1: Int, x2: Int, y2: Int) -> Int
fn is_inside_rectangle(x: Int, y: Int, left: Int, top: Int, right: Int, bottom: Int) -> Bool
fn rectangle_diagonal_squared(width: Int, height: Int) -> Int
fn triangle_area(base: Int, height: Int) -> Int
fn circle_area(radius: Float) -> Float
fn circle_circumference(radius: Float) -> Float
```

## `std.http`

```aco
fn is_url(url: String) -> Bool
fn get(url: String) -> String
fn post_json(url: String, body: String) -> String
fn response(url: String) -> String
fn post_json_response(url: String, body: String) -> String
fn status(url: String) -> Int
fn is_success(url: String) -> Bool
fn error(url: String) -> String
```

## `std.io`

```aco
fn print(value: String)
fn println(value: String)
fn eprint(value: String)
fn eprintln(value: String)
```

## `std.json`

```aco
fn is_valid(text: String) -> Bool
fn escape(text: String) -> String
fn quote(text: String) -> String
fn has_key(text: String, key: String) -> Bool
fn is_null(text: String, key: String) -> Bool
fn get_string(text: String, key: String, fallback: String) -> String
fn get_int(text: String, key: String, fallback: Int) -> Int
fn get_bool(text: String, key: String, fallback: Bool) -> Bool
```

## `std.math`

```aco
fn max(a: Int, b: Int) -> Int
fn min(a: Int, b: Int) -> Int
fn abs(value: Int) -> Int
fn square(value: Int) -> Int
fn clamp(value: Int, lower: Int, upper: Int) -> Int
fn is_even(value: Int) -> Bool
fn is_odd(value: Int) -> Bool
fn cube(value: Int) -> Int
fn sign(value: Int) -> Int
fn in_range(value: Int, lower: Int, upper: Int) -> Bool
fn gcd(left: Int, right: Int) -> Int
fn pow_int(base: Int, exponent: Int) -> Int
fn lcm(left: Int, right: Int) -> Int
fn is_prime(value: Int) -> Bool
fn factorial(value: Int) -> Int
fn ceil_div(value: Int, divisor: Int) -> Int
fn triangular(value: Int) -> Int
fn is_power_of_two(value: Int) -> Bool
fn is_multiple_of(value: Int, divisor: Int) -> Bool
fn round_up_to_multiple(value: Int, step: Int) -> Int
```

## `std.path`

```aco
fn join(left: String, right: String) -> String
fn basename(value: String) -> String
fn dirname(value: String) -> String
fn extension(value: String) -> String
fn stem(value: String) -> String
fn is_absolute(value: String) -> Bool
```

## `std.stats`

```aco
fn sum3(a: Int, b: Int, c: Int) -> Int
fn average2(a: Int, b: Int) -> Int
fn average3(a: Int, b: Int, c: Int) -> Int
fn min2(a: Int, b: Int) -> Int
fn max2(a: Int, b: Int) -> Int
fn min3(a: Int, b: Int, c: Int) -> Int
fn max3(a: Int, b: Int, c: Int) -> Int
fn median3(a: Int, b: Int, c: Int) -> Int
fn sum4(a: Int, b: Int, c: Int, d: Int) -> Int
fn average4(a: Int, b: Int, c: Int, d: Int) -> Int
fn percentage(value: Int, total: Int) -> Int
fn sum2(a: Int, b: Int) -> Int
fn sum5(a: Int, b: Int, c: Int, d: Int, e: Int) -> Int
fn average5(a: Int, b: Int, c: Int, d: Int, e: Int) -> Int
fn range3(a: Int, b: Int, c: Int) -> Int
```

## `std.string`

```aco
fn concat(left: String, right: String) -> String
fn is_empty(value: String) -> Bool
fn repeat_string(value: String, count: Int) -> String
fn surround(value: String, wrapper: String) -> String
fn join3(first: String, separator: String, second: String, third: String) -> String
fn is_not_empty(value: String) -> Bool
fn concat3(first: String, second: String, third: String) -> String
fn join4(first: String, separator: String, second: String, third: String, fourth: String) -> String
fn prefix_suffix(value: String, prefix: String, suffix: String) -> String
```

## `std.timing`

```aco
fn unix_seconds() -> Int
fn sleep_ms(milliseconds: Int)
```

## `std.toml`

```aco
fn is_valid(text: String) -> Bool
fn has_key(text: String, key: String) -> Bool
fn get_string(text: String, key: String, fallback: String) -> String
fn get_int(text: String, key: String, fallback: Int) -> Int
fn get_bool(text: String, key: String, fallback: Bool) -> Bool
```

## `std.units`

```aco
fn celsius_to_fahrenheit(value: Float) -> Float
fn fahrenheit_to_celsius(value: Float) -> Float
fn kilometers_to_miles(value: Float) -> Float
fn miles_to_kilometers(value: Float) -> Float
fn kilograms_to_pounds(value: Float) -> Float
fn pounds_to_kilograms(value: Float) -> Float
fn meters_to_feet(value: Float) -> Float
fn feet_to_meters(value: Float) -> Float
fn liters_to_us_gallons(value: Float) -> Float
fn us_gallons_to_liters(value: Float) -> Float
fn meters_to_kilometers(value: Float) -> Float
fn kilometers_to_meters(value: Float) -> Float
fn centimeters_to_inches(value: Float) -> Float
fn inches_to_centimeters(value: Float) -> Float
fn seconds_to_minutes(value: Float) -> Float
fn minutes_to_seconds(value: Float) -> Float
fn hours_to_seconds(value: Float) -> Float
fn bytes_to_kibibytes(value: Int) -> Int
fn kibibytes_to_bytes(value: Int) -> Int
```

## `std.validation`

```aco
fn is_non_empty(value: String) -> Bool
fn is_in_range(value: Int, lower: Int, upper: Int) -> Bool
fn is_positive(value: Int) -> Bool
fn is_non_negative(value: Int) -> Bool
fn is_negative(value: Int) -> Bool
fn is_zero(value: Int) -> Bool
fn is_non_zero(value: Int) -> Bool
fn is_valid_port(value: Int) -> Bool
fn is_valid_percentage(value: Int) -> Bool
fn is_valid_year(value: Int) -> Bool
fn is_valid_ipv4_octet(value: Int) -> Bool
```

## Updating this page

When the standard library changes, regenerate or compare this inventory against every `.aco` file in the release bundle. Review the behavioral reference separately: signatures alone do not describe sentinel values, side effects, or platform requirements.
