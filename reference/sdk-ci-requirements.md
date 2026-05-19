# UTxO RPC — Mandatory SDK CI Requirements

**Status:** living reference, normative standard owned by the umbrella.
**Source of truth:** this document.

This document describes the *minimum continuous-integration contract* every
SDK submodule must satisfy. Unlike the [SDK API surface](./sdk-api-surface.md),
this is **not** derived from the protocol — it is a standard the umbrella
mandates of each SDK repository. For how each SDK currently measures against
it, see the separate [SDK parity matrix](./sdk-parity.md).

> Authoritative, not derived. This file *is* the requirement. An SDK repo may
> exceed it freely, but may not fall below it. Changes to the contract are
> made here first, then rolled out to each SDK's workflow.

---

## 1. The mandatory pipeline (the contract)

An SDK's CI is **conformant** when a single workflow in the SDK repository
satisfies all of the following. Anything beyond this is allowed and
encouraged; nothing below it is acceptable.

### Triggers
| Event | Condition |
|---|---|
| `pull_request` | targeting the default branch (`main`) |
| `push` | to the default branch (`main`) |

A PR must not be mergeable without this pipeline having run on its head
commit; the same pipeline re-runs on the resulting `main` push.

### Required jobs
| Job | Obligation |
|---|---|
| **build** | Compile/transpile the codebase from a clean checkout. A build failure fails CI. |
| **test** | Run the SDK's full test suite. A test failure fails CI. |

`build` must precede `test` (test runs against built artifacts where the
language requires it). Both run the language's *common* setup for that
ecosystem — toolchain install, dependency resolution — before the obligated
step.

---

## 2. Per-language realization (the "common steps")

The contract is language-agnostic; its realization is not. Each SDK runs the
canonical, idiomatic steps for its ecosystem. The commands below are the
*expected shape*, not a forced verbatim script — an SDK may use the ecosystem's
standard equivalent.

| SDK | Setup (common) | build | test |
|---|---|---|---|
| rust-sdk | toolchain via `rustup`, cargo registry cache | `cargo build` | `cargo test` |
| go-sdk | `setup-go`, module download | `go build ./...` | `go test ./...` |
| node-sdk | `setup-node`, `npm ci` (or `npm install`) | `npm run build` | `npm test` |
| python-sdk | `setup-python`, dependency sync (`uv`/`poetry`/`pip`) | package import / build step | `pytest` |
| dotnet-sdk | `setup-dotnet`, `restore` | `dotnet build` | `dotnet test` |
| haskell-sdk | GHC/Cabal toolchain, `cabal update` | `cabal build` | `cabal test` |

Notes:
- "Common setup" means the steps every project in that ecosystem performs
  regardless of this contract (toolchain + dependencies). It is mandatory only
  insofar as `build`/`test` cannot run without it.
- Lint/format/typecheck (`clippy`, `gofmt`, `dotnet format`, `ruff`, …) are
  **recommended, not required** by this contract. An SDK may gate on them; the
  umbrella does not mandate them here.

---

## 3. Conventions

- **One pipeline, one source.** The build+test obligation lives in a single
  workflow file per SDK so the contract is auditable at a glance.
- **Clean checkout.** CI must not depend on developer-local state; the build
  must succeed from a fresh clone with only the declared toolchain.
- **Spec dependency.** CI builds against whatever `spec`-generated package the
  SDK manifest pins. Keeping that pin current is governed by the
  [`bump-sdk-spec`](../skills/bump-sdk-spec/SKILL.md) procedure, not this
  contract — but the bump is only "done" when this pipeline is green on it.
- **Required, not advisory.** Branch protection on the SDK repo should make
  the pipeline a required check. The umbrella states the requirement; the SDK
  repo enforces it.

### Recommended but not mandated
Matrix builds across language versions/OSes, dependency caching, coverage
reporting, release/publish automation. These are *aspirational*; their
adoption — like API parity — is tracked in the
[parity matrix](./sdk-parity.md), not required here.

---

## 4. Maintenance

- This is the **normative contract** — change it here first, then propagate to
  each SDK's workflow. A divergence between an SDK's CI and this file is a
  defect in the SDK, not in this doc.
- Conformance status of each SDK against this contract belongs in
  [`sdk-parity.md`](./sdk-parity.md), not here. Keep the two separate: this
  doc answers "what CI must an SDK run?", the matrix answers "what does each
  SDK's CI do today?".
- This artifact stays in the wrapper (not `spec/`): it is a cross-cutting
  umbrella policy over the SDK repos, unrelated to the protocol definition.
- Umbrella-scoped; per-SDK CI quirks (runners, ecosystem flags) belong in that
  SDK's repo, provided the contract above still holds.
