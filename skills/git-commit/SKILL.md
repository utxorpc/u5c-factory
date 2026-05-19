---
name: git-commit
description: Create a git commit for the staged/working changes in this repo using Conventional Commits, with no co-authorship trailer. Use whenever the user asks to commit changes in u5c-factory.
---

# Git commit (Conventional Commits, no co-author trailer)

Repo-wide commit convention for u5c-factory. Produces a Conventional Commits
message and **omits any `Co-Authored-By` trailer** (this overrides the default
environment guidance for this repo).

## When to use

- The user asks to commit changes in this umbrella repo.
- Not for landing changes inside a submodule — those follow the submodule's
  own conventions and are committed in that repo (see `AGENTS.md`).

## Prerequisites

- There are changes to commit (`git status --short`).
- You know which paths to stage. When in doubt, stage explicitly rather than
  `git add -A` — submodule pointer moves should be deliberate, not incidental.

## Format

```
<type>(<optional scope>): <description>

<optional body — why, not what; wrap ~72 cols>

<optional footer — BREAKING CHANGE:, refs>
```

- **type** (required): `feat`, `fix`, `docs`, `refactor`, `chore`, `test`,
  `build`, `ci`, `perf`, `style`, `revert`.
- **scope** (optional): the affected area, e.g. `spec`, `reference`,
  `skills`, an SDK name (`rust-sdk`), or `submodules`.
- **description**: imperative mood, lower-case, no trailing period,
  ≤ ~72 chars.
- **breaking change**: append `!` after type/scope (`feat(spec)!: ...`)
  and/or a `BREAKING CHANGE:` footer.
- **No `Co-Authored-By:` line. No tool/assistant attribution trailer.**

## Steps

1. Review what will be committed: `git status --short` and
   `git diff --staged` (or `git diff` for unstaged).
2. Stage the intended paths explicitly.
3. Pick the `type` from the dominant intent of the change; add a `scope` if it
   sharpens it.
4. Write the message per the format above. If the change spans unrelated
   concerns, prefer splitting into multiple commits over a vague message.
5. Commit without the co-author trailer:
   `git commit -m "<subject>" -m "<body>"` (multiple `-m` for body/footer).
6. Confirm: `git log --oneline -1` shows the expected subject and the message
   contains no `Co-Authored-By` line.

## Verification

- `git log -1 --format=%B` prints a Conventional-Commits-shaped message with
  **no** `Co-Authored-By` / assistant attribution trailer.
- `git status` reflects the intended staged paths only (no accidental
  submodule pointer bumps).

## Notes

- This deliberately overrides the environment's default "end commit messages
  with Co-Authored-By" guidance — the user set this as the repo convention.
- Umbrella-scoped. Submodule commits are out of scope.
