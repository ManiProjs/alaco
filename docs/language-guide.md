# Alaco language guide

Alaco is a small, statically checked language that compiles through generated C++. Source files use the `.aco` extension. This guide describes the syntax implemented by the current compiler; it is not a promise that every feature of Rust or C++ is supported.

## A first program

```alaco
function main() {
    let language = "Alaco";
    println("Hello from " + language + "!")
}
```

`main` is the program entry point. `std.io` is available without an import, so `print`, `println`, `eprint`, and `eprintln` can be called directly.

## Source files and basic syntax

Alaco source files use `.aco`. Newlines end statements, so semicolons are unnecessary (and accepted only as optional separators for compatibility). Blocks use `{` and `}`. Comments use `//` for the remainder of a line.

The compiler is statically checked: it analyzes types and names before generating C++. A program can therefore fail during parsing, semantic analysis, C++ compilation, or execution. A successful `alaco check` means the Alaco front end accepted the program; it does not build the native executable.

## Types and variables

The built-in types include:

| Type | Meaning | Example |
| --- | --- | --- |
| `int` | Integer | `let count: int = 3;` |
| `float` | Floating-point number | `let temperature: float = 21.5;` |
| `bool` | `true` or `false` | `let ready: bool = true;` |
| `string` | Text | `let title: string = "Hello";` |

Type annotations can be omitted when the compiler can infer a variable's type. Use `var` when a binding must be reassigned; `let` bindings are immutable:

```alaco
var total: int = 0;
total = total + 1;
```

Annotate a binding when you want to make the intended type explicit or inference needs help. Alaco does not implicitly convert unrelated types; integer values may be widened to `float` in compatible numeric contexts. Lowercase primitive type names (`int`, `float`, `bool`, `string`, `void`) are canonical; their capitalized spellings remain accepted for older code.

### Composite types

Type annotations support nested parameterized types, heterogeneous tuples, and callable values:

```alaco
struct AccountSummary {
    recent_ids: Array<int>,
    labels: Map<string, string>,
    unique_tags: Set<string>,
    selected_id: Option<int>,
    lookup_result: Result<string, int>,
    position: (float, float),
    predicate: fn(int, string) -> bool,
}

function process_ids(ids: Array<Option<int>>) {
    // Work with ids here.
}
```

The currently recognized type constructors are `Array<T>`, `Set<T>`, `Map<K, V>`, `Option<T>`, and `Result<T, E>`. The compiler checks constructor names, argument counts, nested types, and rejects `void` in value positions. Tuple types use `(T, U, ...)` and require at least two elements. Function types use `fn(ArgumentType, ...) -> ReturnType`; values with a function type can be called, with argument count and argument types checked at compile time. The native backend represents these types using C++ standard-library containers, `std::optional`, `std::variant`, `std::tuple`, and `std::function`.

`Option<T>` has runtime values: `none` creates an empty option and `some(value)` wraps a value. Read `.is_some` or `.is_none` to inspect it, and `.value` to access a present value; reading `.value` from `none` raises a catchable runtime error. A `none` value can be assigned to an explicitly typed `Option<T>` binding. `Result<T, E>` currently provides type annotations and native layout only. Alaco does not yet provide generic function or generic struct declarations, collection literals and operations, `Result` constructors or pattern matching, or trait declarations. Enums and exhaustive matching are supported for named, payload-free variants; payload-bearing variants and generic declarations remain future work.

### Enums and exhaustive `match`

Declare an enum with one or more named variants. Use `EnumName.Variant` to construct a value and to name a case in a match:

```alaco
enum Direction {
    North,
    East,
    South,
    West,
}

function direction_name(direction: Direction): string {
    match direction {
        Direction.North => { return "north"; }
        Direction.East => { return "east"; }
        Direction.South => { return "south"; }
        Direction.West => { return "west"; }
    }
}

function main() {
    println(direction_name(Direction.West));
}
```

Every enum match must cover every variant, or include `_` as a fallback. Missing variants and unknown or duplicate variant cases are compile-time errors. Enum variants currently carry no payloads; `match` remains a statement, and each arm contains a statement or block.

## Functions

Declare functions with `function`, typed parameters, and an optional return type after a colon. Functions without a return type do not return a value. The older `fn` keyword and `->` return annotation remain accepted while existing projects migrate.

Functions with a non-`void` return type must return a value on every statically recognized path. The analyzer recognizes direct returns and `if`/`else` branches that both return; it does not assume that a loop will run or terminate.

### Runtime errors

Raise a runtime error with `throw` and handle it with `try`/`catch`:

```alaco
function read_name(): string {
    try {
        throw "name is unavailable";
    } catch error {
        println("Using fallback: ${error}");
        return "guest";
    }
}
```

The expression after `throw` must have type `string`. A `catch name` binding contains the error message as an immutable string and is visible only inside that catch block. Native output uses `std::runtime_error`; JavaScript output uses `Error`. Uncaught errors terminate the program through the target runtime. For a non-`void` function, a `try`/`catch` counts as returning only when both blocks are statically guaranteed to return; throwing alone does not satisfy the function's return requirement.

```alaco
function add(left: int, right: int): int {
    return left + right
}

function main() {
    println(add(20, 22))
}
```

## Conditions and operators

`if` supports an optional `else` block. Conditions use `bool` expressions:

```alaco
if score >= 10 {
    println("Passed");
} else {
    println("Try again");
}
```

The implemented binary operators include arithmetic (`+`, `-`, `*`, `/`, `%`), comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`), logical `and` / `or`, and compound assignments (`+=`, `-=`, `*=`, `/=`). `+=` accepts either compatible numeric operands or two `string` values, matching the behavior of `+`. Logical operators require `bool` operands; `and` binds more tightly than `or`. The unary `!` operator negates a boolean, and unary `-` negates a number. Parentheses can group expressions.

Equality is defined for `int`, `float`, `bool`, and `string`; comparing structs, shell values, or `void` is rejected. `%` currently requires two `int` operands.

### Reactive derived values

Use `derive` when a local value should stay synchronized with other variables:

```alaco
var price = 100;
var quantity = 3;
derive total = price * quantity;
derive discount_available = total >= 250;

println(total); // 300
quantity = 1;
println(total); // 100
println(discount_available); // false
```

Alaco records the identifiers used by each derived expression and inserts updates after statements that assign to those dependencies. Derived values can depend on earlier derived values; updates follow declaration order so a chain is recalculated from its sources outward. Unaffected derived expressions are not recalculated. Circular dependencies among derived declarations in the same lexical block are compile-time errors. Declare source variables before the derived values that use them. This is synchronous compiler-generated recalculation, not a background observer or cross-thread reactive system. See `examples/derived_values.aco`.

### Remembering values and reacting with `when`

`previous name` reads the value of a local variable immediately before its most recent assignment. Its initial previous value is the value established by the variable's declaration (or the function argument's initial value). It currently supports `int`, `float`, `bool`, and `string` locals/parameters.

```alaco
var score = 0;
when score != previous score {
    println("Score changed: ${score}");
}
score = 1;
score = 2;
```

`when condition { ... }` checks the condition immediately when registered and after each subsequent statement in the same lexical block, running the block whenever the condition is true at one of those checkpoints. It is synchronous—not a background task. Put the `when` after the variables it observes have been declared. A watcher declared inside a block is scoped to that block.

### Timed variables

Give a local variable a deadline with `for` between its name and optional type:

```alaco
let cache for 20 seconds = "temporary";
var session for 1 minute: string = "active";
```

Supported units are `ms`/`milliseconds`, `s`/`seconds`, and `m`/`minutes`. When the deadline arrives, Alaco clears the stored value automatically. Reading, assigning, or using `previous` after expiry raises a runtime error instead of silently recreating the value. Timed variables currently support `int`, `float`, `bool`, and `string`. The deadline is fixed at declaration; reassignment does not renew it. The compiler uses a small timer thread per timed variable, so use this for short-lived values rather than thousands of tiny-lived locals.

Use `==` and `!=` for equality; a condition must evaluate to `bool`, so numeric values are not implicitly treated as true or false. `+` adds compatible numeric values and concatenates strings. When in doubt, keep expressions simple and use parentheses to make grouping clear.

### Pipe expressions

The forward pipe operator `|>` passes its left-hand value as the first argument to a function on the right. Any arguments already written in a call follow the piped value. Pipes associate from left to right:

```alaco
function double(value: int): int {
    return value * 2;
}

function add(left: int, right: int): int {
    return left + right;
}

function main() {
    let result = 4 |> double |> add(3);
    println(result); // 11
}
```

This is equivalent to `add(double(4), 3)`. Pipe inputs are checked against the target function's parameter types before C++ is generated.

## Loops and match

Use `loop (repeat count)` to execute a block a fixed number of times:

```alaco
loop (repeat 3) {
    println("Again");
}
```

`match` selects a branch by matching a value against patterns. `_` is the fallback pattern:

```alaco
match answer {
    1 => println("one")
    2 => println("two")
    _ => println("something else")
}
```

The currently documented loop form is a fixed-count repeat loop. `match` supports literal patterns, qualified enum variants, and `_` as a catch-all. Enum matches are checked for exhaustiveness. Enum variants do not carry payloads yet. Do not assume `break` or payload-bearing pattern matching are available unless the compiler's current grammar explicitly supports them.

## Structs

Structs group named, typed fields. Field access uses dot notation.

```alaco
struct User {
    name: string,
    age: int,
}

function main() {
    let user = User {
        name: "Ada",
        age: 36,
    };
    println(user.name);
    println(user);
}
```

You can destructure selected fields into local variables. By default, each binding keeps the field's name; use `field: binding` to rename it. Fields you do not list are ignored.

```alaco
let Point { x, y: vertical } = point;
println(x);
println(vertical);
```

## Imports

Import a standard-library or local module. A single-component local import resolves to a sibling `.aco` file (`import helpers;` loads `helpers.aco`); dotted local imports map intermediate components to directories and the final component to a `.aco` file (`import helpers.greetings;` loads `helpers/greetings.aco`). Paths are resolved relative to the importing `.aco` file. The final component is the namespace used to call its functions by default. Use `as` to choose a different namespace.

To call a specific function without the module prefix, use a named import:

```alaco
import hello from helpers;

function main() {
    println(hello("Alaco"));
}
```

This loads `helpers.aco` and makes `hello` directly callable. Named imports currently support one function name per import statement. Try it with `alaco run examples/modules/named_import.aco` from the repository root.

```alaco
import std.math;
import helpers;
import helpers.greetings;
import helpers.math_tools as maths;

function main() {
    println(math.clamp(12, 0, 10));
    println(greetings.hello("Alaco"));
    println(maths.square(9));
}
```

For these local imports, the importing directory can contain `helpers.aco` as well as `helpers/greetings.aco` and `helpers/math_tools.aco` under a `helpers/` directory. Imported modules can import other modules relative to their own file; dependencies are loaded first. Duplicate aliases and cyclic imports are reported as errors. Run `alaco run examples/modules/main.aco`, `alaco run examples/modules/simple_import.aco`, or `alaco run examples/modules/named_import.aco` from the repository root to try the examples.

See the [standard-library reference](standard-library.md) for available modules and their behavior.

### State, events, workflows, and schema evolution

Alaco includes first-party modules for stateful applications: `std.state` enforces a registered transition graph at runtime, `std.events` provides cursor-based in-process event streams, `std.workflow` persists checkpoints to an append-only journal for restart recovery, and `std.schema` wraps JSON payloads in named, versioned envelopes. These APIs are implemented by compiler-provided runtime hooks for both native and JavaScript output; applications do not need third-party packages for the core mechanisms.

For persistent relational data, `std.db` provides SQLite connections, parameterized SQL, transactions, and versioned migrations. Native builds require SQLite 3 development headers and a linkable library; JavaScript output uses Node.js 22.5+'s built-in `node:sqlite` module. See the [database reference](standard-library.md#stddb--sqlite-databases) for the API and backend requirements.

The first release is intentionally explicit rather than magical. State graphs and event streams are process-local. Workflow code owns scheduling, retries, idempotency, and step execution; the journal makes checkpoints recoverable but does not provide exactly-once side effects. Schema migrations are ordinary Alaco functions that transform one version to another; the compiler does not generate field validators or automatically run a migration registry. See the [standard-library reference](standard-library.md#stdstate--checked-state-machines) and `examples/state_systems.aco` for the APIs and their current guarantees.

### Import resolution notes

- `import helpers;` resolves to `helpers.aco` next to the importing file.
- `import helpers.greetings;` resolves to `helpers/greetings.aco` next to the importing file.
- `import helpers as util;` gives the module a local alias.
- `import hello from helpers;` imports one named function without the module prefix.

Local imports are resolved relative to the source file containing the import, not necessarily the shell's current directory. Imports whose first component names a dependency in `Alaco.toml` resolve from that installed package. Imported modules are loaded before their dependents. Cyclic imports and duplicate module aliases are errors. Named imports currently select a single function per statement.

### String interpolation

Insert a variable into a string with `${name}` syntax. Literal text and variable values are combined at runtime:

```alaco
function main() {
    let name = "Alaco";
    let version = 2026;
    println("Hello, ${name}! Welcome to Alaco ${version}.");
}
```

Interpolation currently supports variable names (including numeric and boolean variables), not arbitrary expressions. To try it, run `alaco run examples/string_interpolation.aco` from the repository root. Strings without `${...}` keep their existing behavior, including JSON text containing braces.

Interpolation is useful when a message combines fixed text with a value. For a calculation, compute the result first and interpolate the resulting variable:

```alaco
function main() {
    let subtotal = 18;
    let quantity = 3;
    let total = subtotal * quantity;
    println("Total: ${total}");
}
```

## Package manifests

Alaco projects use `Alaco.toml` for package metadata. A minimal manifest has a `[package]` table with `name`, semantic `version`, `edition = "2026"`, and an optional `.aco` `entry` path (default `src/main.aco`). `[dependencies]` accepts Git tables (`git` and an optional pinned `rev`) and local `path` tables. Run `alaco add <git-repo>` to install a library and recursively install its dependencies, pinning each Git dependency to a commit SHA in its parent package's manifest. Import libraries with `import package_name;` or `import package_name.module;`. There is no registry, and cyclic dependency graphs are rejected.

Use `alaco new my_app` to create a project scaffold, or `alaco init` to initialize the current directory. Inside a project, commands without an explicit source path use the configured entry point where applicable. Keep application code under `src/` and reusable local modules beside the source file that imports them, or in matching subdirectories for dotted module paths.

## Shell expressions

Alaco supports a shell block expression. It runs the block as a command and returns a `ShellResult` value, which can be printed:

```alaco
function main() {
    let result = shell {
        echo "Hello from a command"
    };
    print(result);
}
```

Shell commands are platform-dependent. Only run shell commands you trust.

The shell block is an operating-system integration, not a portable replacement for standard-library functions. Prefer `std.fs`, `std.path`, or other standard modules when they cover the task. A shell block's command syntax and available programs depend on the host environment.

## Compiler checks and diagnostics

The analyzer checks function argument types and catches common mistakes such as misspelled functions, module functions, and struct fields. Diagnostics use stable `alaco::analysis::*` codes; source labels target the expression or declaration responsible for the error. Unknown types are highlighted at the type name, type mismatches at the relevant expression, and duplicate declarations at the repeated declaration. Some whole-program errors, such as a missing `main`, have no source span to highlight. Use `alaco check file.aco` to parse and analyze without building or running the program.

### Recommended development loop

1. Write a small `.aco` file and keep one behavior in focus.
2. Run `alaco check path/to/file.aco` to catch syntax and type/name errors early.
3. Run `alaco run path/to/file.aco` to compile and execute it.
4. Use `alaco build path/to/file.aco` when you want a persistent native executable.
5. Add a focused program under `tests/` and run `alaco test` when the behavior should stay covered.

For command-line options, run `alaco --help`. For standard-library signatures and failure behavior, consult the [standard-library reference](standard-library.md). The compiler is evolving, so this guide documents the implemented subset rather than promising compatibility with another language.
