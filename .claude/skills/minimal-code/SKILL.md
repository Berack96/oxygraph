---
name: minimal-code
description: Self-check of scope and minimality on your own diff, to run before declaring a code-change task in this repository done.
---

Before declaring a code-change task in this repository done, re-read your `git diff` (staged and unstaged) line by line.

1. For every changed line, ask what part of the request it corresponds to. If you can't answer, remove it.
2. **Scope**: no file touched that wasn't needed for the task; no line reformatted, renamed, or import reordered without being asked; nothing "fixed in passing" in unrelated nearby code.
3. **Minimality**: no generalizations, abstractions, optional parameters, edge-case handling, logging, or config that nobody asked for; nothing added "because it'll be needed eventually."
4. **Leftovers**: no debug code, unused imports, temp files, or accidental changes.
5. If you find an unrequested bug in code you touched for other reasons, don't fix it on your own initiative: report it at the end of the work (file, line, why it's a bug) and let the user decide.
6. Outcome: either the diff is already minimal, or a precise list of lines/files to remove before closing the task.
