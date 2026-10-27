# Reactive state and timed values

Alaco has a small set of constructs for responding to local state changes: `previous`, `when`, and timed variables. They are useful for simple state transitions and short-lived values, but they are not a general asynchronous UI framework. Understanding when code runs is essential.

## Observe a local value with `when`

`when condition { ... }` registers a watcher. The condition is checked when the watcher is registered and again after each subsequent statement in the same lexical block. The body runs at each checkpoint where the condition evaluates to `true`.

```aco
fn main() {
    let mut score = 0;

    when score != previous score {
        println("Score is now ${score}");
    }

    score = 1;
    score = 2;
}
```

The watcher is synchronous. It does not run on a background task, and it does not subscribe to changes made by an external thread or service. In this example, the body is evaluated at statement checkpoints after the assignments, so it observes the updated value.

### What `previous` means

`previous name` reads the value held immediately before that local variable's most recent assignment. Before the first reassignment, its previous value is the value from the declaration; for a function parameter, it is the argument's initial value.

```aco
let mut temperature = 20;
let old_temperature = previous temperature;
temperature = 24;
let before_update = previous temperature; // 20
```

`previous` currently supports local variables and parameters of type `Int`, `Float`, `Bool`, and `String`. Keep the name local and explicit; it is not a history log and does not retain every past value.

## Keep watchers scoped and predictable

- Declare the values a watcher reads before the `when` statement.
- A watcher declared inside a block is scoped to that block.
- A `when` condition is checked at defined statement checkpoints, not continuously between instructions.
- Use `when` for local, synchronous reactions; use explicit functions and ordinary control flow for work that must happen exactly once.
- Do not assume `when` creates an event loop, background task, or browser-style reactive component.

Because the condition is checked after statements in its lexical block, a watcher may run more than once while a block executes. Make the body safe to repeat.

## Give a variable a deadline

Timed variables are declared with `for` after the variable name. The deadline is measured from declaration and does not reset when the variable is assigned again.

```aco
fn main() {
    let cache for 20 seconds = "warm";
    let mut session for 1 minute: String = "active";

    println(cache);
    session = "refreshed";
    println(session);
}
```

Supported duration units are `ms` / `milliseconds`, `s` / `seconds`, and `m` / `minutes`. Timed variables currently support `Int`, `Float`, `Bool`, and `String`.

When a deadline expires, reading, assigning, or using `previous` on the value raises a runtime error. Expiry does not silently reset the variable to its original value. Keep uses within the lifetime and avoid storing references to values whose lifetime has expired.

### When timed variables are appropriate

Timed values can model a short-lived token, temporary status, or cache entry with a fixed lifetime. They are not a full cache implementation: there is no renewal-on-access behavior, eviction policy, or asynchronous expiry callback. The native implementation uses a timer thread per timed variable, so avoid creating thousands of short-lived timed locals.

## Combine state updates with pipes

The `|>` operator passes its left-hand value as the first argument to the function on the right. Additional arguments remain after the piped value, and pipes associate left to right.

```aco
fn double(value: Int) -> Int {
    return value * 2;
}

fn add(left: Int, right: Int) -> Int {
    return left + right;
}

fn main() {
    let result = 4 |> double |> add(3);
    println(result); // 11
}
```

This is equivalent to `add(double(4), 3)`. A pipe is ordinary synchronous function composition; it does not schedule work or make a value reactive by itself.

## Target support

Timed variables and shell expressions are not supported by the JavaScript backend. The native C++ backend is the relevant target for the timed-variable examples above. See [Compiler targets](/guide/targets) and [Current limitations](/reference/limitations) before depending on target-specific behavior.
