# AGENTS.md — UTxO RPC (u5c) Factory

> **Canonical agent guidance for this repository.** Other agent files
> (`CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`) intentionally
> contain no guidance of their own — they point here. Edit this file, not them.

## What this repo is

**UTxO RPC** ("u5c") is an RPC interface for interacting with UTxO-based
blockchains. This repository (`u5c-factory`) is the **umbrella / wrapper repo**:
it does not contain product code itself. Instead it aggregates the
specification and every language SDK as **pinned git submodules**, and provides
a single place for cross-cutting concerns that span more than one component.

## Repository map

Each submodule is an independent repo with its own history, issues, and
(usually) its own `AGENTS.md` / `README.md`. **Always consult the submodule's
own docs for component-specific conventions** — this file only routes you there.

| Path               | Component        | Look here for                                  |
|--------------------|------------------|------------------------------------------------|
| `spec/`            | Specification    | Protocol, `.proto` schemas, RPC semantics      |
| `sdks/rust-sdk/`   | Rust SDK         | Rust client/bindings                           |
| `sdks/node-sdk/`   | Node / JS SDK    | JavaScript/TypeScript client (browser & node)  |
| `sdks/go-sdk/`     | Go SDK           | Go client/bindings                             |
| `sdks/haskell-sdk/`| Haskell SDK      | Haskell client/bindings                        |
| `sdks/python-sdk/` | Python SDK       | Python client/bindings                         |
| `sdks/dotnet-sdk/` | .NET SDK         | C# / .NET client/bindings                      |

## Routing rules — where does a change belong?

- **Protocol, schema, `.proto`, RPC method or message shape** → `spec/`.
  This is the source of truth; SDKs are generated from / track it.
- **Language-specific binding, client behavior, or SDK bug** →
  `sdks/<lang>-sdk/`. Make the change in that submodule's own repo.
- **A change that touches multiple components together** (e.g. a spec change
  plus the SDK updates that follow it) → coordinate per-submodule changes in
  their own repos, then bump the pointers here (see below).
- **Repo-wide conventions, this routing map, agent guidance** → this file.

## Reference artifacts

Cross-cutting, derived reference docs live in `reference/`. These are
synthesized from the spec and submodules — authoritative-by-derivation, not
hand-authored truth. When a doc disagrees with `spec/` or an SDK, trust the
submodule and update the doc.

- `reference/sdk-api-surface.md` — the ideal, language-agnostic SDK API
  surface (normative-by-derivation; "what should an SDK expose?"). Re-derive
  after any `spec/` bump.
- `reference/sdk-parity.md` — cross-SDK feature-parity matrix ("what does
  each SDK expose today?"). Re-derive after any `spec/` bump or SDK pointer
  update.

## Skills

Reusable, cross-cutting agent procedures live in `skills/<name>/SKILL.md` —
the canonical, agent-agnostic store. Each is a self-contained directory with a
`SKILL.md` (YAML frontmatter + body); copy `skills/_template/SKILL.md` to start
a new one. Claude Code discovers them via `.claude/skills` (a symlink to
`skills/`); agents without skill auto-loading should read the relevant
`SKILL.md` directly.

Skills here are **umbrella-level / cross-cutting only** (e.g. bumping a
submodule pointer, regenerating bindings after a spec change, cutting a
coordinated release). Procedures specific to one SDK belong in that submodule's
own repo, consistent with the routing rules above.

## Working with submodules

- Submodules are **pinned to a specific commit**. Editing files inside a
  submodule changes only that submodule's working tree; it does not update this
  repo until you commit the new pointer.
- Bumping a submodule (pointing it at a newer upstream commit) is an
  **explicit, reviewable commit in this repo**. Do not bump pointers
  incidentally — only when intentionally updating that component.
- Clone with `git clone --recurse-submodules`; refresh with
  `git submodule update --init --recursive`.
