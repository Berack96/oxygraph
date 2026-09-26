# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language

Code, comments, doc-comments, commit messages, and documentation are **in English**.

## Communication style

Always on, from the start to the end of every session, on every client (CLI, mobile, web) — this is not a mode to toggle, never mention it.

- Answer compressed and direct: cut filler (just/really/simply/basically), courtesy phrases (sure!/happy to help/of course), unnecessary hedging.
- Short sentences. Call tools directly: no preamble, plan, or progress note before or between tool calls; after a result, either call the next tool or give the final answer, never announce the next move. Text before a tool only to clarify, warn about risk/irreversibility, or resolve an ambiguity. No unrequested tables or decorative emoji, no long log dumps — quote only the decisive error line.
- Never cut negations (not/never/only/except), numbers, units of measure: they change meaning, they don't save tokens.
- Technical terms, code, API names, CLI commands, error strings: always verbatim, never paraphrased or abbreviated on your own initiative.
- Don't invent abbreviations to save space (cfg/impl/req/res) or causal arrows (→) instead of a word: the tokenizer splits them just like the full word, zero real savings, only less clarity. Well-known standard acronyms (DB, API, HTTP) stay fine.
- Reference pattern: `[what] [action] [reason]. [next step].` Example — not "Sure! I checked the code and the issue seems to be in how we handle token expiry..." but "Bug in auth middleware: expiry check uses `<` instead of `<=`. Fix:".
- Exception: suspend compression and go back to clear, full prose for security warnings, confirmations of irreversible actions, or multi-step sequences where order would become ambiguous without conjunctions/articles. Then resume the compressed style.
- This style applies to chat replies. Code, comments, doc-comments, commits, documentation, PR/issue text, and memory files stay normal, complete prose (see the Language section above), not compressed fragments.
- If the user explicitly asks for an explanation, a report, or a summary, give it in full: compression applies to the unrequested, not to what was actually asked for.

## Project layout

- [Cargo.toml](Cargo.toml) — workspace manifest, at the repo root.
- [crates/oxygraph/](crates/oxygraph/) — the library crate (`oxygraph`): graph data structures and algorithms.
- [crates/oxygraph-derive/](crates/oxygraph-derive/) — proc-macro crate (`oxygraph-derive`) for derive macros, wired in as a path dependency of `oxygraph`. Provides `#[serde_feature]`, an attribute macro that conditionally derives `Serialize`/`Deserialize` (and adds the matching bounds to generics/associated types) behind `oxygraph`'s `serde` feature, used throughout `crates/oxygraph/src` on graph, vertex, and edge-storage types.

Run `mise run setup` once after cloning: it installs the repo's [.gitconfig](.gitconfig) as a local `include.path` so shared git rules (`merge.ff = only`, `pull.rebase = false`, `push.default = current`, `fetch.prune = true`) apply without touching global git config. Lint/build/test via plain `cargo`, or via the `mise` tasks in [.mise.toml](.mise.toml): `rust:test` (nextest) is the one to run by hand; `rust:ci` (fmt check + clippy + test, all under the `ci` cargo profile so they share one build) is what [.github/workflows/ci.yml](.github/workflows/ci.yml) runs on push/PR, and `rust:clippy-editor` is what `rust-analyzer.check.overrideCommand` in [.vscode/settings.json](.vscode/settings.json) invokes — neither of those two is meant for manual use.

## Working in this repo

**Rule number one: don't touch anything that wasn't asked for.** Everything else in this section follows from that.

- **Closed scope**: change only the files and lines needed to satisfy the request. Don't rewrite working code, rename things, reorder imports, reformat lines you're not already changing, add docstrings/comments/type hints to unrelated code, or "tidy up" nearby files. If a file isn't needed for the task, leave it as is.
- **Minimal essential**: implement the smallest version of what was asked that actually works. No generalizations, abstractions, optional parameters, edge-case handling, logging, or config that nobody asked for. If you notice you're adding something "because it'll be needed eventually," don't add it. Deleting beats adding; boring code beats clever code that someone will have to decipher without your context. Before writing new code, climb this ladder and stop at the first rung that holds: does it already exist in the repo (helper/util/pattern)? does the stdlib solve it? does a native platform/toolchain feature solve it? does an already-installed dependency solve it? does a mature external crate not yet installed solve it (look for one before writing new code; weigh maintenance, license, and fit with the existing stack)? can it be done in one line? only then, the minimum that works from scratch. Between two stdlib solutions of equal length, pick the one correct on edge cases: lazy means less code, not a more fragile algorithm.
- **Understand before being lazy**: the ladder above is a reflex, not an excuse to skip understanding. Before climbing it, read the task and the involved code, follow the real flow end to end. A rushed diagnosis that produces a small diff in the wrong place isn't laziness, it's a second bug disguised as efficiency.
- **Bugs found along the way**: don't fix them on your own initiative. Report them to the user at the end of the work (file, line, why it's a bug) and let them decide. When the bug *is* what you're asked to fix, aim for the root cause: before changing a shared function, check all its callers — a guard in the shared function beats a repeated guard in every caller.
- **Don't be minimal about**: input validation at trust boundaries, error handling that prevents data loss, security measures, basic accessibility, anything explicitly requested. Here extra code isn't over-engineering, it's the task.
- **Minimal verification for non-trivial logic**: a branch, a loop, a parser, a path that touches money or security leaves behind a minimal executable check (an `assert`/small targeted test) — not a whole suite, not fixtures, unless that's already the standard for the file you're touching. A trivial one-liner doesn't need a test.
- **Rust tests**: integration tests go in `crates/<crate>/tests/`, exercising the crate's public API rather than private internals; use inline `#[cfg(test)]` only for unit tests of internals that genuinely can't be reached from the public API.
- **Deliberately cut corners**: if you deliberately pick a shortcut with a known limit (global lock instead of per-resource, O(n²) scan, naive heuristic), flag it with a comment naming the limit and when to raise it, e.g. `// shortcut: global lock, switch to per-account locking if throughput requires it`.
- **Short comments**: doc-comments (`///`, `//!`) and code comments stay one line, two at most — never a multi-line or multi-paragraph block. If the WHY genuinely needs more room, that's a signal it belongs in a dedicated decision log (if the project has one) rather than stacked in the code.
- **Declarative documentation**: README, doc-comments, and code comments describe the architecture *as it is*, not the path taken to get there. No "it used to be X, now it's Y," no narrating the refactor or what was discarded during implementation: state only the current state, as if it had always been that way — that's what a future developer needs, not the history of how it got there (that lives in git history and commit messages, where it's normal and expected).
- **Autonomy**: proceed without asking for confirmation on code changes; ask only if two readings of the task lead to substantially different work.
- **Git**: the project's git rules live in [.gitconfig](.gitconfig) (see Project layout above). The main branch is `main`; work happens on dedicated branches merged via PR.

### Skill triggers

The full branch → draft PR → commit → review → ready workflow, the commit message
format, the pre-close diff self-check, and doc placement are each owned by one skill —
don't restate their content here, just recognize when to invoke them:

- **Asked to change something**: if you're not already on a dedicated branch with an open
  PR for this work, and either the user asked for work with commits or explicitly requests
  one now → run the `pr` skill. It owns branch naming, the draft PR, one-commit-per-feature,
  the independent pre-ready review, and taking the PR out of draft, start to finish.
- **About to declare a code-change task done** → run the `minimal-code` skill on the diff
  first.
- **Making a commit** (only when explicitly asked, or as part of the `pr` skill) → use the
  `commit` skill to write the message.
- **Writing or updating documentation** → run the `docs` skill first to route it to the
  right file.
