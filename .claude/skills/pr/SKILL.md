---
name: pr
description: Manages the full flow of a feature/fix with commits in this repository, from branch creation to a pull request ready for review. Use when a piece of work with commits is explicitly requested.
---

This skill only applies when work with commits was explicitly requested — don't open branches, make commits, or open PRs on your own initiative.

1. **Branch**: create a branch from `origin/main` (or the branch you're starting from, if different) with a name that reflects what the feature/fix does.
2. **Checklist and first commit**: define the checklist of items to implement, build the first one and make a dedicated commit for it, then push the branch — you need at least one commit of difference from the base, GitHub won't let you open a PR between two identical branches. Commit message: in English, Conventional Commits style (`<type>: <subject>`, lowercase, imperative, no trailing period; types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`), a body only if it's needed to explain the *why* of a non-obvious choice.
3. **Open the PR as draft**: immediately open the PR in **draft** state from the branch targeting the starting branch.
   - Title: Conventional Commits, short, in English.
   - Body, in English: **Summary** (2-3 lines, the PR's current state, not the history of how it got there); if the work stems from an issue (or you're asked to link one), a standalone line with the closing keyword (e.g. `Closes #<number>`) — never just a text mention — and only if this PR's base branch is the repository's default branch (otherwise the issue won't auto-close on merge: say so explicitly in that case); **Changes**, the full checklist with the first item already checked; **Note** (optional) for things explicitly left out of scope.
4. **Implementation**: implement the remaining checklist items one at a time (partially too, if it makes sense to split them): a dedicated commit for each, same format as step 2, and check it off in the PR description.
5. **Requests that expand the work**: if during the work a request comes in to change the feature or add something else — a small deviation from what's already in progress → implement it in the same PR; an actual refactor or something too far from the work in progress → ask for confirmation on how to proceed instead of deciding on your own. When the change goes into the PR, update the description to reflect the current state, not the path taken to get there (no "it used to be X, then we added...").
6. **Pre-ready review**: before marking the PR ready, do a small review of the work by delegating it to an independent subagent, without the context of whoever wrote the changes — never in-process: whoever wrote the changes already has the context and risks missing bugs, duplication, typos, or logic issues that a fresh reader would catch. Ask it to read the touched files in full (not just the diff) and report the problems found. Fix small findings directly in the PR; for large ones, ask for confirmation on how to proceed.
7. **Ready**: remove references to the future ("will be done", "still to do"...) from the description and mark the PR ready for review (take it out of draft).

If the platform requires an attribution footer in the PR body, add it at the end.
