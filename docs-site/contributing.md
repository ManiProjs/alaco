# Contributing to Alaco documentation

Good documentation is part of the language implementation. A page should help someone complete a task, understand why it works, recognize its failure modes, and find the next relevant topic. Prefer tested facts over optimistic descriptions of planned behavior.

## Before editing a page

1. Identify the exact compiler release or source revision the page describes.
2. Check the implementation or a runnable example before documenting command flags, syntax, return values, and platform behavior.
3. Search for an existing page on the same topic. Extend it rather than creating a second page that will drift.
4. Check the [current limitations](/reference/limitations) page so examples do not imply unsupported features.

The website and documentation live separately from the compiler source branch. A documentation change must not silently change compiler behavior or claim that a feature exists merely because it is planned.

## Recommended page structure

For a task-oriented guide, use this order where it makes sense:

1. **Purpose and scope:** what the page helps the reader do, and what it does not cover.
2. **Prerequisites:** required release, platform tools, project layout, or permissions.
3. **Working example:** the smallest complete example, with the command and expected result.
4. **Explanation:** why the syntax or workflow works; explain important trade-offs.
5. **Failure modes:** recognizable symptoms, likely causes, and safe ways to verify them.
6. **Related topics:** links to the relevant guide, reference, and limitations pages.

Reference pages should prioritize exact signatures, accepted values, defaults, return behavior, platform differences, and version boundaries. Avoid turning a reference page into a long introductory tutorial.

## Accuracy rules

- Distinguish `alacoup` (compiler installation and version management) from `alaco` (the compiler and project CLI).
- Label behavior that is release-specific. Do not present a current limitation as a permanent language design decision.
- Do not claim Rust, C++, JavaScript, or another language's compatibility unless it has been verified in Alaco itself.
- Document failure behavior, including sentinel return values and external tool requirements.
- Mark examples that intentionally fail or perform filesystem, network, or shell operations.
- Never invent CLI flags, environment variables, standard-library functions, or error messages to make an example look complete.
- Prefer a short reproducible example over a large excerpt that hides the relevant behavior.

## Verify changes locally

From the website repository:

```sh
npm install
npm run docs:dev
```

Before submitting a change, build the documentation:

```sh
npm run docs:build
git diff --check
```

For code examples, run the exact Alaco command documented by the page against the release or source revision the page describes. A successful VitePress build checks page generation; it does not prove that Alaco code samples compile.

## Keep the wiki navigable

- Give each page one clear subject and a descriptive title.
- Link to the canonical explanation instead of copying the same instructions into several pages.
- Add new pages to the VitePress sidebar and link them from the most relevant existing pages.
- Use symptom-oriented headings in troubleshooting pages so local search can find them.
- Keep limitations close to the feature they qualify, and link to the central limitations page.
- Update examples and version labels when a compiler release changes behavior.

## Report a documentation problem

When reporting an error, include the page URL, the compiler version involved, the incorrect or confusing statement, and what you observed instead. For a broken example, include the exact command and its output. Do not include private credentials or tokens in reports.
