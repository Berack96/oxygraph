---
name: commit
description: Generates the message for a git commit in this repository, when explicitly asked to make a commit.
---

Generate the message for the next commit in this git repository. Only commit when explicitly asked, never on your own initiative.

1. Look at what's staged (`git diff --staged`) and, if you need the full picture, also `git status` / `git diff`. If nothing is staged but there are changes relevant to the task just finished, consider staging them — never with `git add -A`, only add the files actually touched for this task.
2. If the staged changes mix unrelated things (several logically distinct changes together), stop and flag it: each commit should correspond to one logical change, not bundle different things into one.
3. Write the message in **English**, Conventional Commits style: `<type>: <subject>`, lowercase, imperative, no trailing period. Types: `feat` (new feature), `fix` (bug fix), `docs` (documentation), `refactor` (restructuring without behavior change), `test`, `chore`.
4. Add a body only if it's needed to explain the *why* of a non-obvious choice — don't repeat the *what*, that's visible from the diff.
5. Don't mention the conversation, the task, or an AI assistant in the message: it describes the code, not the context it was written in.
