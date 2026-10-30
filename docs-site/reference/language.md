# Language reference

This page is an index to the language constructs implemented by the current Alaco compiler. It is deliberately conservative: syntax not listed here should not be assumed to work just because a similar feature exists in Rust, C++, or another language.

## Source and declarations

| Construct | Form | Details |
| --- | --- | --- |
| Source file | `.aco` | Alaco source files use the `.aco` extension. |
| Comment | `// comment` | Starts a single-line comment. |
| Function | `fn name(arg: Type) -> Type { ... }` | Parameters and explicit return types are supported. Omit `-> Type` for a function with no value return. |
| Immutable binding | `let name = value;` | A normal `let` binding cannot be reassigned. |
| Mutable binding | `let mut name = value;` | Use `mut` when the binding will be reassigned. |
| Type annotation | `let name: Int = 1;` | An annotation may be omitted when inference can determine the type. |
| Struct | `struct Name { field: Type }` | Structs contain named, typed fields. |
| Import | `import std.math;` | Import a standard-library or local module. |
| Alias | `import helpers.math_tools as maths;` | Give an imported module a local namespace. |
| Named import | `import hello from helpers;` | Import one function name from a module. |

The built-in scalar types are `Int`, `Float`, `Bool`, and `String`. Struct types are also supported. Do not assume generics, enums, arrays, maps, or arbitrary user-defined type features are available unless a current language page documents them.

## Expressions and operators

The documented operators are:

| Group | Operators | Notes |
| --- | --- | --- |
| Arithmetic | `+`, `-`, `*`, `/`, `%` | `+` also concatenates compatible strings; `%` currently requires `Int` operands. |
| Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` | Conditions must produce `Bool`. |
| Logical | `and`, `or`, `!` | Logical binary operators require `Bool`; `and` binds more tightly than `or`. |
| Compound assignment | `+=`, `-=`, `*=`, `/=` | Update a mutable binding. |
| Pipe | `|>` | Pass the left value as the first argument to a function on the right. Pipes associate left to right. |
| Unary numeric | `-` | Negate a numeric value. |

Parentheses group expressions. Equality is defined for `Int`, `Float`, `Bool`, and `String`; the current analyzer rejects equality comparisons for structs, shell values, and `Void`.

## Control flow

| Construct | Form | Details |
| --- | --- | --- |
| Conditional | `if condition { ... } else { ... }` | The condition must be `Bool`; `else` is optional. |
| Fixed-count loop | `loop (repeat count) { ... }` | Repeat a block a fixed number of times. |
| Conditional loop | `loop (while condition) { ... }` | Check the condition before each iteration. |
| Match | `match value { pattern => expression }` | Literal patterns and `_` catch-all are documented. |
| Watcher | `when condition { ... }` | Synchronous checks at statement checkpoints in the containing lexical block. |
| Previous value | `previous name` | Read a supported local variable's value before its latest assignment. |

Timed bindings use `let name for duration = value;`; the supported units are milliseconds, seconds, and minutes. Timed values and shell expressions are not supported by JavaScript output. See [Reactive state and timed values](/guide/reactive-state).

## Structs and interpolation

Struct fields are accessed with dot notation (`user.name`). Struct construction uses named fields. Selected fields can be destructured into local bindings, including renamed bindings such as `let Point { x, y: vertical } = point;`.

String interpolation uses `${name}` to insert a variable's value. The current implementation does not support arbitrary expressions inside the interpolation braces.

## Shell expressions

Native programs can use a shell block expression:

```aco
fn main() {
    let result = shell {
        echo "hello"
    };
    println(result);
}
```

The block runs a host command and produces a `ShellResult`. Shell syntax and available commands depend on the operating system and environment. Only execute shell commands you trust. Shell expressions are not supported by JavaScript output.

## Detailed guides

- [Types, functions, and expressions](/guide/language-types)
- [Control flow and data](/guide/language-control-flow)
- [Modules and strings](/guide/language-modules)
- [Reactive state and timed values](/guide/reactive-state)
- [Projects and runtime](/guide/language-projects)
- [Current limitations](/reference/limitations)
