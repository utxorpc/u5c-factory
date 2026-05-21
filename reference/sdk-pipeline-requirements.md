# UTxO RPC — Mandatory SDK Pipeline Requirements

**Status:** living reference, normative standard owned by the umbrella.
**Source of truth:** this document.

Every SDK submodule must run two automated pipelines in its own repository: a
**CI pipeline** that gates every commit, and a **release pipeline** that gates
every published artifact. This document is the normative contract for both.
Unlike the [SDK API surface](./sdk-api-surface.md), it is **not** derived from
the protocol — it is a standard the umbrella mandates of each SDK repository.
For how each SDK currently measures against it, see the
[SDK parity matrix](./sdk-parity.md).

> Authoritative, not derived. This file *is* the requirement. An SDK repo may
> exceed it freely, but may not fall below it. Changes to the contract are
> made here first, then rolled out to each SDK's workflows.

---

## 1. The CI pipeline

Runs on every change, before it merges. Conformant when a single workflow in
the SDK repository satisfies all of the following.

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
language requires it).

---

## 2. The release pipeline

Runs when a version is published. Conformant when a single workflow — separate
from the CI workflow — satisfies all of the following.

### Trigger
| Event | Condition |
|---|---|
| `push` | a tag matching `v*` |

Releasing is triggered by pushing a **version tag** — nothing else. The tag is
the one human action; everything after it is automated. `workflow_dispatch`
MAY be added for manual dry-runs, but it MUST NOT publish without a tag
context (the tag is checked against the manifest version — see §3).

### Tag format
`v<MAJOR>.<MINOR>.<PATCH>` — `v` followed by a SemVer version. A SemVer
pre-release suffix is permitted (`v1.7.0-alpha`, `v0.2.0-rc.1`).

### Required stages
| Stage | Obligation |
|---|---|
| **build** | Compile/transpile the codebase from a clean checkout of the tagged commit. |
| **test** | Run the SDK's full test suite. |
| **publish** | Push the package to its registry (§4). |

The stages run **in order** and each gates the next: `publish` MUST NOT run
unless `build` and `test` both succeeded. There is no partial publish — a
failed release leaves nothing on the registry. The release re-runs the full
build and test suite (§1) so a tag can never publish an untested artifact.

---

## 3. Versioning

The committed **package manifest is the single source of version truth** —
rust `Cargo.toml`, node `package.json`, python `pyproject.toml`, dotnet
`.csproj`, haskell `*.cabal`. A release is cut by bumping that version in a
normal commit, then pushing a matching `v<version>` tag.

- The release workflow MUST verify that the pushed tag equals the manifest
  version and **fail fast** if they disagree — before build, test, or publish.
- The workflow MUST NOT edit the manifest version to paper over a mismatch,
  and MUST NOT publish a version that differs from the manifest.
- A failed release publishes nothing. The fix is a corrected commit and a new
  tag — tags are never moved or re-pointed.

Haskell carve-out: the haskell SDK packages use independent `0.0.x` numbering
that does not track the `v<semver>` line of the other SDKs. The rule is
unchanged — the tag must match the `.cabal` version — and both
`utxorpc-client` and `utxorpc-server` must carry that same version.

---

## 4. Per-language realization

The contracts above are language-agnostic; their realization is not. Each SDK
runs the canonical, idiomatic steps for its ecosystem. The commands below are
the *expected shape*, not a forced verbatim script.

### CI build & test
| SDK | Setup (common) | build | test |
|---|---|---|---|
| rust-sdk | toolchain via `rustup`, cargo registry cache | `cargo build` | `cargo test` |
| go-sdk | `setup-go`, module download | `go build ./...` | `go test ./...` |
| node-sdk | `setup-node`, `npm ci` (or `npm install`) | `npm run build` | `npm test` |
| python-sdk | `setup-python`, dependency sync (`uv`/`poetry`/`pip`) | package import / build step | `pytest` |
| dotnet-sdk | `setup-dotnet`, `restore` | `dotnet build` | `dotnet test` |
| haskell-sdk | GHC/Cabal toolchain, `cabal update` | `cabal build` | `cabal test` |

### Release registry & auth
Each SDK publishes to the **same registry, with the same auth mechanism, that
the `spec` repo's codegen uses for that language**. Secret names MUST match the
`spec` repo's codegen secret names verbatim, so the organization configures one
secret set.

| SDK | Registry | Auth mechanism | Secret name | Publish command |
|---|---|---|---|---|
| rust-sdk | crates.io | API token (`CARGO_REGISTRY_TOKEN` env) | `CARGO_REGISTRY_TOKEN` | `cargo publish` |
| go-sdk | Go module proxy (git-tag native) | default `GITHUB_TOKEN` | — | git tag + GitHub Release + proxy notify |
| node-sdk | npmjs.org | OIDC trusted publishing (`id-token: write`, no token) | — | `npm publish --access public` |
| python-sdk | PyPI | API token, username `__token__` | `PYPI_REGISTRY_TOKEN` | `poetry publish` |
| dotnet-sdk | nuget.org | API key | `NUGET_REGISTRY_TOKEN` | `dotnet nuget push --skip-duplicate` |
| haskell-sdk | Hackage | API key (`HACKAGE_KEY` env) | `HACKAGE_REGISTRY_TOKEN` | `stack upload` |

Notes:
- "Common setup" means the steps every project in that ecosystem performs
  regardless of this contract (toolchain + dependencies). Lint/format/typecheck
  (`clippy`, `gofmt`, `ruff`, …) are **recommended, not required**.
- **go-sdk** has no package registry — a Go module *is* its git tag. Its
  publish stage creates the GitHub Release and notifies the Go module proxy;
  it needs no registry secret.
- **node-sdk** uses npm OIDC trusted publishing — no static token. The publish
  job needs `permissions: id-token: write`, and the package's trusted publisher
  must be configured on npmjs.org before the first release.
- **Idempotency:** re-running a release for an already-published version MUST
  NOT overwrite the published artifact. Registries that reject duplicate
  versions by policy satisfy this for free; where the registry does not
  (NuGet), the publish step uses `--skip-duplicate`.

---

## 5. Conventions

- **One workflow file per pipeline.** The CI obligation lives in one workflow
  file; the release obligation in another. Two concerns, two files, each
  auditable at a glance.
- **Clean checkout.** Both pipelines must run from a fresh clone with only the
  declared toolchain — no developer-local state.
- **Spec dependency.** Both build against whatever `spec`-generated package the
  SDK manifest pins; keeping that pin current is governed by
  [`bump-sdk-spec`](../skills/bump-sdk-spec/SKILL.md), not this contract — but
  a bump is only "done" when both pipelines are green on it.
- **Required, not advisory.** Branch protection should make CI a required
  check; tag protection should restrict who can push a `v*` tag. The umbrella
  states the requirement; the SDK repo enforces it.

### Recommended but not mandated

Matrix builds across language versions/OSes, dependency caching, coverage
reporting, `workflow_dispatch` dry-run paths, generated release notes, signed
tags. These are *aspirational*; their adoption is tracked in the
[parity matrix](./sdk-parity.md), not required here.

---

## 6. Maintenance

- This is the **normative contract** — change it here first, then propagate to
  each SDK's workflows. A divergence between an SDK's pipelines and this file
  is a defect in the SDK, not in this doc.
- Conformance status of each SDK belongs in [`sdk-parity.md`](./sdk-parity.md),
  not here. This doc answers "what pipelines must an SDK run?"; the matrix
  answers "what does each SDK run today?".
- This artifact stays in the wrapper (not `spec/`): it is a cross-cutting
  umbrella policy over the SDK repos, unrelated to the protocol definition.
- Umbrella-scoped; per-SDK quirks (runners, ecosystem flags) belong in that
  SDK's repo, provided the contract above still holds.
