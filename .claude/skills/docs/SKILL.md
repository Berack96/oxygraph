---
name: docs
description: Routes a documentation update in this repository to the right file. Use when asked to update documentation, or when a change just made seems to require it.
---

Before writing any line of documentation in this repository, figure out where it belongs: writing it in the wrong place creates duplication that drifts over time.

1. **Crate-specific information** (a crate's own API surface, module layout, commands specific to `crates/<crate>/`) → goes in that crate's own README/doc-comments, not in the repo's general agent instructions file.
2. **Operational information** for anyone working in the repo (where things live, constraints, workflow) → can go in the general agent instructions file, but only if: the project just underwent a large structural change (a new crate under `crates/`, a new or renamed `mise` task, a change to the main stack or tools); or a statement already written there no longer matches reality (verified in code, not by hearsay). Outside those two cases, the general instructions file stays untouched, even for a local fix or refactor.
3. Before adding any section, check whether the information already exists elsewhere in the repository. If it does, point there instead of repeating it — don't create a second copy of the same information.
4. Write the current state, not the history: never "it used to be X, now it's Y," only "the function does X." A document that contradicts itself or narrates its own history is worse than an incomplete one.
