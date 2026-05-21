# u5c-factory

Umbrella / wrapper repository for **UTxO RPC** ("u5c") — an RPC interface for
interacting with UTxO-based blockchains.

This repo contains no product code. It aggregates the specification and every
language SDK as pinned git submodules and serves as the entry point for
cross-cutting concerns. See [`AGENTS.md`](./AGENTS.md) for the repository map
and routing rules.

## Layout

| Path               | Repository                                   |
|--------------------|----------------------------------------------|
| `spec/`            | https://github.com/utxorpc/spec              |
| `sdks/rust-sdk/`   | https://github.com/utxorpc/rust-sdk          |
| `sdks/node-sdk/`   | https://github.com/utxorpc/node-sdk          |
| `sdks/go-sdk/`     | https://github.com/utxorpc/go-sdk            |
| `sdks/haskell-sdk/`| https://github.com/utxorpc/haskell-sdk       |
| `sdks/python-sdk/` | https://github.com/utxorpc/python-sdk        |
| `sdks/dotnet-sdk/` | https://github.com/utxorpc/dotnet-sdk        |

## Cloning

```sh
git clone --recurse-submodules https://github.com/utxorpc/u5c-factory.git
```

If you already cloned without submodules:

```sh
git submodule update --init --recursive
```

To pull the latest umbrella state and refresh submodule pointers:

```sh
git pull
git submodule update --init --recursive
```

## Reference artifacts

Cross-cutting reference docs live in [`reference/`](./reference). They are
synthesized from the spec and SDK submodules — authoritative by derivation, not
hand-authored truth. When a doc disagrees with `spec/` or an SDK, trust the
submodule and update the doc.

| Document | What it is |
|----------|------------|
| [`reference/sdk-api-surface.md`](./reference/sdk-api-surface.md)       | The ideal, language-agnostic SDK API surface — "what *should* an SDK expose?" Re-derive after any `spec/` bump. |
| [`reference/sdk-pipeline.md`](./reference/sdk-pipeline.md) | The mandatory CI and release pipeline contract every SDK repo must satisfy. A normative standard owned by the umbrella. |
| [`reference/sdk-parity.md`](./reference/sdk-parity.md)                 | Cross-SDK feature-parity matrix — "what does each SDK expose *today*?" Re-derive after any `spec/` bump or SDK pointer update. |

## Skills

Reusable, cross-cutting agent procedures live in [`skills/<name>/SKILL.md`](./skills) —
the canonical, agent-agnostic store. Claude Code discovers them via the
`.claude/skills` symlink; other agents should read the relevant `SKILL.md`
directly. Skills here are **umbrella-level only**; procedures specific to one
SDK belong in that submodule's own repo. Copy
[`skills/_template/SKILL.md`](./skills/_template/SKILL.md) to start a new one.

| Skill | What it does |
|-------|--------------|
| [`bump-sdk-spec`](./skills/bump-sdk-spec/SKILL.md)     | Bump every SDK submodule's UTxO RPC spec / proto-gen dependency to the latest published release. |
| [`check-sdk-parity`](./skills/check-sdk-parity/SKILL.md) | Re-derive the cross-SDK feature-parity matrix (RPC, CI, and release conformance) from the submodules and rewrite `reference/sdk-parity.md`. |
| [`start-pr-queue`](./skills/start-pr-queue/SKILL.md)   | Open one PR per changed submodule and hand back an ordered merge plan — typically after a `bump-sdk-spec` run. |
| [`git-commit`](./skills/git-commit/SKILL.md)         | Commit anywhere in the u5c project using Conventional Commits with no co-authorship trailer. |
