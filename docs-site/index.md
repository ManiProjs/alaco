# Alaco documentation

Alaco v0.2.0 is a small, statically checked programming language implemented in Rust. Its compiler analyzes `.aco` source files and supports native executable builds through generated C++ plus an evolving JavaScript transpilation target.

<div class="vp-doc-intro-actions">
  <a class="vp-doc-primary" href="./guide/getting-started">Get started <span>→</span></a>
  <a class="vp-doc-secondary" href="./guide/language">Explore the language <span>→</span></a>
</div>

## Follow a path

<div class="vp-doc-cards">
  <a href="./guide/getting-started"><strong>01 · Set up</strong><span>Install Alaco, create a project, and run your first program.</span></a>
  <a href="./guide/language"><strong>02 · Learn syntax</strong><span>Understand types, functions, control flow, structs, and imports.</span></a>
  <a href="./reference/standard-library"><strong>03 · Use the library</strong><span>Explore the APIs available to Alaco programs.</span></a>
  <a href="./reference/language"><strong>04 · Language reference</strong><span>Look up the supported syntax and language constructs.</span></a>
  <a href="./guide/editor-support"><strong>05 · Improve your workflow</strong><span>Connect the language server to your editor.</span></a>
  <a href="./reference/releases"><strong>06 · Install Alaco</strong><span>Manage compiler versions with AlacoUp.</span></a>
  <a href="./guide/testing"><strong>07 · Test programs</strong><span>Run focused `.aco` tests and keep regressions visible.</span></a>
  <a href="./cookbook/"><strong>08 · Cookbook</strong><span>Practical recipes for CLI arguments, JSON, HTTP, files, and loops.</span></a>
  <a href="./guide/troubleshooting"><strong>09 · Troubleshoot</strong><span>Find fixes by symptom, from installation to runtime and editor issues.</span></a>
  <a href="./guide/dependencies"><strong>10 · Add dependencies</strong><span>Use local packages, Git sources, version constraints, and lockfiles.</span></a>
  <a href="./guide/build-cache"><strong>11 · Build faster</strong><span>Understand cache decisions and force a fresh native build.</span></a>
</div>

## At a glance

- **Source files:** `.aco`
- **Compiler implementation:** Rust
- **Native output:** generated C++ compiled with a system C++ compiler
- **JavaScript output:** generated JavaScript for Node.js, with an evolving set of supported features
- **Editor tooling:** Language Server Protocol, including diagnostics, workspace-aware completion, scope-aware local navigation and rename, hover, and code actions

The current stable release is **v0.2.0**. See the [release downloads](/reference/releases) for platform archives and the VS Code extension.

## Help improve the docs

Alaco's documentation is intended to be a practical, community-maintained knowledge base. See the [documentation guidelines](/contributing) for how to add tested examples, explain failure modes, and keep pages accurate as the compiler evolves.

Alaco is actively evolving. These pages describe the current implementation, including limitations, rather than promising compatibility with Rust, C++, or a complete language specification.
