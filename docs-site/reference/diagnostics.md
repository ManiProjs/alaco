# Diagnostic reference

Alaco reports structured diagnostics with a machine-readable identifier, a human-readable message, a source location when available, and a short help suggestion. The identifier is useful when searching the documentation or reporting a compiler issue; the message and highlighted span explain what this particular source file needs.

Use `alaco explain path/to/file.aco` to run the checker and display diagnostics with source context. Use `alaco check` when you only need a success/failure check. A successful check covers Alaco parsing and semantic analysis; it does not prove that generated C++ compiles or that runtime behavior is correct.

## Observed diagnostic identifiers

The following identifiers and message shapes were reproduced with the locally active compiler binary. The wording can become more specific as the compiler evolves.

| Identifier | Meaning | Typical fix |
| --- | --- | --- |
| `alaco::parser::unexpected_token` | The parser found a token that is not valid at the current grammar position. | Read the expected-token hint, then inspect the token immediately before the highlighted location. Check delimiters and declaration syntax. |
| `alaco::analysis::undefined_variable` | A referenced name is not visible in the current scope. | Check spelling, declaration order, imports, and whether the name is qualified with the correct module alias. |
| `alaco::analysis::analysis_error` | A semantic analysis error that does not have a more specific public identifier in the observed output. | Read the message for the expected and actual types or the rejected construct; inspect the highlighted expression. |

These are examples, not an exhaustive registry. Do not infer that every diagnostic from a compiler stage uses one of these identifiers.

## Read the diagnostic layout

An `explain` diagnostic generally contains:

1. **Identifier:** a compiler namespace and code, such as `alaco::analysis::undefined_variable`.
2. **Headline:** a short summary of the problem.
3. **Location:** the source file, line, and column where the compiler can identify the problem.
4. **Source excerpt:** the relevant line and a marker showing the highlighted range.
5. **Help:** a suggested next step when the compiler has one.

The highlighted span is the compiler's best-known location, not necessarily the only line that contributed to the error. For example, a wrong argument can originate in a declaration several lines earlier.

## Examples

An undefined name:

```aco
fn main() {
    println(missing_name);
}
```

The observed identifier is `alaco::analysis::undefined_variable`, and the help text recommends declaring the name before using it.

A type mismatch:

```aco
fn main() {
    let count: Int = "wrong";
}
```

The observed output uses `alaco::analysis::analysis_error` and explains that `count` expects `Int` but the initializer is `String`. The highlighted range points to the initializer.

An unexpected token:

```aco
fn main( {
}
```

The observed identifier is `alaco::parser::unexpected_token`. In this case the parser reports that it expected a parameter name and found `{`.

## Diagnose by compiler stage

| Stage | What it means | Next step |
| --- | --- | --- |
| Parsing | The source does not match the grammar. | Check delimiters, statement structure, and the expected-token hint. |
| Semantic analysis | A name, type, argument, or construct is invalid in context. | Check declarations, imports, annotations, and the highlighted expression. |
| Native C++ compilation | Alaco accepted the source, but generated C++ or a native dependency failed. | Run `alaco doctor`, keep the first C++ compiler error, and check platform libraries. |
| Runtime | The executable started but encountered an invalid state or external failure. | Reduce the input and inspect file, environment, timer, shell, and network operations. |
| JavaScript runtime | The generated JavaScript failed or differs from native behavior. | Run the generated file with Node.js and check [target compatibility](/reference/compatibility). |

## Reporting a diagnostic problem

Include the exact Alaco release, the command, the complete diagnostic, and the smallest source file that reproduces it. If the diagnostic has a source range, preserve the source formatting so the location remains meaningful. Remove secrets and private paths before sharing logs.

Related pages: [Diagnostics and troubleshooting](/guide/diagnostics), [Troubleshooting by symptom](/guide/troubleshooting), and [Current limitations](/reference/limitations).
