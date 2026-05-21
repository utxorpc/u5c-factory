---
name: check-sdk-parity
description: Re-derive the cross-SDK feature-parity matrix by inspecting every SDK submodule and rewrite reference/sdk-parity.md to match. Use after a spec/ bump, an SDK pointer change, or whenever the parity doc may be stale.
---

# Check SDK parity

Cross-cutting umbrella procedure: inspect each SDK submodule at the commit this
repo pins, work out what it actually exposes, and rewrite
`reference/sdk-parity.md` so every cell reflects reality. The parity matrix is
**derived, not authoritative** — this skill regenerates the derivation.

## When to use

- After a `spec/` submodule bump — new RPCs may exist, version cells go stale.
- After any `sdks/*` pointer change — method, CI, release, or capability cells
  may move.
- After a [`bump-sdk-spec`](../bump-sdk-spec/SKILL.md) run, to refresh the
  "Spec / proto-gen version per SDK" table (that skill's final step delegates
  here).
- Whenever the matrix is suspected stale and you want it re-grounded.

Do **not** use this to change what an SDK *should* expose — normative "should"
lives in `reference/sdk-api-surface.md` and
`reference/sdk-pipeline-requirements.md`. This skill only records what each
SDK *does* today.

## Prerequisites

- Submodules initialized: `git submodule update --init --recursive`.
- Know the pinned spec release: `git -C spec describe --tags`.
- Read the normative references — they define every row/column the matrix
  is scored against:
  - `reference/sdk-api-surface.md` — the RPC methods and cross-cutting
    capabilities that make up the matrix rows.
  - `reference/sdk-pipeline-requirements.md` — the mandatory CI and release
    pipeline contract scored in the "CI conformance" and "Release conformance"
    tables.
- Per-SDK build toolchains are **not** required — this is source inspection,
  not a build. (`bump-sdk-spec` covers build verification.)

## Steps

Work one SDK at a time. The matrix columns are: rust-sdk, go-sdk, node-sdk,
python-sdk, dotnet-sdk, haskell-sdk.

1. **Spec / proto-gen version.** For each SDK, read its manifest and record the
   spec dependency and version (rust `Cargo.toml` → `utxorpc-spec`; go `go.mod`
   → `github.com/utxorpc/go-codegen`; node `package.json` → `@utxorpc/spec`;
   python `pyproject.toml` → `utxorpc-spec`; dotnet `*.csproj` →
   `Utxorpc.Spec`; haskell `*.cabal` → `utxorpc`, independent `0.0.x`
   numbering). Score each ✅ current / ⚠️ / ❌ behind against the pinned spec
   tag, with a footnote when a registry release is missing or unverified.

2. **CI conformance.** Inspect each SDK's `.github/workflows/`. A cell is ✅
   only for a CI workflow (not release/publish-only) running on the mandated
   trigger. Score PR trigger, main-push trigger, build job, test job, and the
   overall Conformant verdict per `sdk-pipeline-requirements.md` §1. Footnote
   anything partial (e.g. test-only, tag-triggered-only).

3. **Release conformance.** Inspect each SDK's release workflow — the
   `.github/workflows/` file that publishes a package, distinct from CI.
   Against `sdk-pipeline-requirements.md` §2, score the tag-push trigger, the
   build / test / publish stages (✅ dedicated gated stage, ⚠️ incidental,
   ❌ absent), and whether the registry and publish secret name match the
   `spec` repo (§4). Footnote anything partial.

4. **RPC method coverage.** For each service group in the matrix (Query,
   Submit, Sync, Watch), search the SDK's client/public surface for each
   method listed in `sdk-api-surface.md`. Score ✅ implemented & idiomatic /
   ⚠️ partial or non-idiomatic / ❌ missing. Be conservative: ✅ only for an
   exposed, idiomatic public method.

5. **Cross-cutting capabilities.** Score each capability row (TLS, auth,
   streaming, sync/async, FieldMask, pagination, retry, batch submit,
   high-level helpers) the same way; use `—` where not applicable to the
   language.

6. **Rewrite `reference/sdk-parity.md`.** Update every table, the pinned-spec
   line in the header, and all footnotes. Keep the existing structure, legend,
   and the "Derived, not authoritative" disclaimer. Do not invent rows — the
   row set comes from `sdk-api-surface.md`; if the spec added a new RPC, add
   the row and score it.

## Verification

- Every cell traces to something observed in a submodule at its pinned commit —
  no cell left from a previous derivation without re-checking.
- The pinned-spec version in the header matches `git -C spec describe --tags`.
- The version table matches each SDK's manifest exactly.
- `git diff reference/sdk-parity.md` is reviewable: each changed cell
  corresponds to a real change in the submodules since the last derivation.
- Legend, footnotes, and the derived-not-authoritative note are intact.

## Notes

- Inspecting submodules does not move any pointer — this skill only reads SDK
  working trees and writes `reference/sdk-parity.md` in the umbrella.
- Cells are only meaningful against the SDK commits this umbrella pins;
  re-derive whenever those pointers move.
- When a cell disagrees with a submodule, the submodule wins — fix the cell.
- Umbrella-scoped only. Why a given SDK lacks a method is a question for that
  SDK's own repo, not this matrix.
