# Getting started

This guide takes you from a fresh machine to a running Alaco program. AlacoUp manages Alaco compiler installations; you use the `alaco` command to create, check, run, and build programs. You do not need to clone Alaco or install Rust.

## 1. Install prerequisites

You need:

- **AlacoUp**, the Alaco toolchain manager. Its bootstrap installer installs AlacoUp and then uses it to install the latest stable Alaco compiler and standard library.
- **A C++ compiler** supported by your platform for native `build` and `run` commands.
- **Node.js** only if you want to execute programs transpiled to JavaScript.

Install AlacoUp. On macOS and x86-64 Linux, review the [installer script](https://github.com/ManiProjs/alacoup/blob/main/install.sh), then run:

```sh
curl --fail --location --proto '=https' --tlsv1.2 \\
  https://raw.githubusercontent.com/ManiProjs/alacoup/main/install.sh | sh
```

On Windows PowerShell, review the [PowerShell installer](https://github.com/ManiProjs/alacoup/blob/main/install.ps1), then run `irm https://raw.githubusercontent.com/ManiProjs/alacoup/main/install.ps1 | iex`. The installer installs AlacoUp and uses it to install the latest stable Alaco compiler. Rust is not required. If the commands are not found afterward, add `~/.alacoup/bin` to your `PATH` and open a new shell.

Verify the installation and check your native build tools:

```sh
alacoup --version
alacoup doctor
alaco --help
```

Use `alacoup install VERSION` to install a specific compiler version, `alacoup list` to list installed versions, and `alacoup use VERSION` to activate one. AlacoUp provides the `alaco` launcher used for everyday compiler commands. See [Release downloads](/reference/releases) for details.

## 2. Create a project

```sh
alaco new hello
cd hello
```

The generated project includes `Alaco.toml` and a source entry point. A typical entry point looks like this:

```text
fn main() {
    let language = "Alaco";
    println("Hello from " + language + "!");
}
```

`std.io` is available by default, so you do not need to import it for `print` or `println`.

## 3. Check, run, and build

```sh
alaco check
alaco run
alaco build
```

- `alaco check` parses and analyzes source without producing a native executable.
- `alaco run` builds a temporary native executable, runs it, and cleans up the temporary output.
- `alaco build` leaves the executable in the project's `target/` directory unless you choose an explicit output path.

A successful check means the Alaco front end accepted the program. Native compilation can still fail later if the generated C++ or the platform toolchain encounters a problem.

## 4. Make an intentional mistake

Try changing `println` to `printl`, then run `check` again. The semantic analyzer should report an unknown function and may suggest a correction. This is the central development loop: edit, check, run, and repeat.

## 5. Transpile to JavaScript

From the `hello` project created above, transpile its entry point:

```sh
alaco transpile src/main.aco --output hello.js
node hello.js
```

The output path is optional; without `--output`, Alaco writes a sibling `.js` file. JavaScript output is an evolving target. Check the [target guide](/guide/targets) before relying on target-specific standard-library behavior.

## Next steps

- Read the [language guide](/guide/language).
- Learn the [project workflow](/guide/project-workflow).
- Browse the [CLI reference](/reference/cli) and [standard library](/reference/standard-library).
