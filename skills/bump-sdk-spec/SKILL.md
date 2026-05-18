---
name: bump-sdk-spec
description: Traverse every SDK submodule in the u5c-factory and bump its UTxO RPC spec / proto-gen dependency to the latest published release. Use when a new spec version ships and SDKs need to be brought current.
---

# Bump SDK spec version

Cross-cutting umbrella procedure: raise the `spec`-generated dependency in each
SDK submodule to the latest published version, per registry. Each SDK consumes
proto-gen code from a *different package on a different registry with a
different version scheme* — there is no single edit that covers all of them.

## When to use

- A new `spec` release has been published and one or more SDKs lag behind it
  (see `reference/sdk-parity.md` → "Spec / proto-gen version per SDK").
- Use **after** the `spec/` submodule pointer has been bumped and the
  corresponding codegen packages have been published to their registries.

Do **not** use this to: regenerate codegen itself (that lives in `spec` /
`go-codegen` repos), or to bump non-spec dependencies.

## Prerequisites

- Submodules initialized: `git submodule update --init --recursive`.
- Per-SDK toolchains available for the verification step (`cargo`, `go`,
  `npm`/`pnpm`, `python`+`uv`/`pip`, `dotnet`, `cabal`). Skip verification for
  any toolchain not installed and note it.
- Know the **target spec release** (e.g. the tag the `spec/` submodule is
  pinned to: `git -C spec describe --tags`). "Latest" = newest published
  package on each registry that corresponds to that spec release.

## Steps

For each SDK, resolve the latest published version on its registry, edit the
manifest, then verify. Work one SDK at a time; commit per SDK or once at the
end (see Notes).

1. **rust-sdk** — `sdks/rust-sdk/Cargo.toml`
   - Latest: `cargo search utxorpc-spec` (or crates.io).
   - Edit: `utxorpc-spec = { version = "<X>" }`.
   - Verify: `cargo update -p utxorpc-spec && cargo build` in `sdks/rust-sdk`.

2. **go-sdk** — `sdks/go-sdk/go.mod`
   - Dependency is `github.com/utxorpc/go-codegen` (not a "spec" package).
   - Latest: `go list -m -versions github.com/utxorpc/go-codegen`.
   - Edit: `cd sdks/go-sdk && go get github.com/utxorpc/go-codegen@<vX> && go mod tidy`.
   - Verify: `go build ./...`.

3. **node-sdk** — `sdks/node-sdk/package.json`
   - Latest: `npm view @utxorpc/spec version`.
   - Edit the `@utxorpc/spec` entry; refresh lockfile with the project's
     package manager (`npm install` / `pnpm install`).
   - Verify: `npm run build` (or the project's build script).

4. **python-sdk** — `sdks/python-sdk/pyproject.toml`
   - Latest: `pip index versions utxorpc-spec` (or PyPI).
   - Edit: `utxorpc-spec = "<X>"`; refresh lock if one exists.
   - Verify: install deps and import the package.

5. **dotnet-sdk** — `sdks/dotnet-sdk/src/Utxorpc.Sdk/Utxorpc.Sdk.csproj`
   - Package is `Utxorpc.Spec` (note: may use a `-alpha` suffix).
   - Latest: `dotnet package search Utxorpc.Spec` / NuGet.
   - Edit the `<PackageReference Include="Utxorpc.Spec" Version="<X>" />`.
   - Verify: `dotnet build` from `sdks/dotnet-sdk`.

6. **haskell-sdk** — `sdks/haskell-sdk/client/utxorpc-client.cabal`
   - Package is `utxorpc`, with its **own independent `0.0.x` numbering** —
     it does NOT track `spec` tag numbers. Map the target spec release to the
     matching `utxorpc` Hackage release before editing.
   - Latest: check Hackage for `utxorpc`.
   - Edit every `utxorpc >= <X> && < <Y>` bound (appears multiple times).
   - Verify: `cabal build` from `sdks/haskell-sdk`.

7. **Refresh the parity reference.** Update the version table and spec-lag
   theme in `reference/sdk-parity.md` to reflect the new versions. If method
   coverage changed because newer proto-gen exposes new RPCs (e.g.
   `ReadState`), re-derive the matrix.

## Verification

- Each edited manifest shows the new version; lockfiles regenerated.
- Each SDK whose toolchain is available builds cleanly against the new
  dependency. Record any SDK skipped (toolchain absent) or failing (with the
  error) — do not silently leave a broken bump.
- `reference/sdk-parity.md` "Spec / proto-gen version per SDK" table matches
  reality; run `git -C spec describe --tags` and confirm SDKs are now current
  (or document why one cannot be — e.g. registry package not yet published).

## Notes

- **Editing a manifest changes only that submodule's working tree.** It does
  not move the submodule pointer recorded in this umbrella repo. Decide
  explicitly whether each bump is committed *inside the SDK repo* (a real SDK
  change, usually a PR there) versus just bumping the pointer here. This skill
  prepares the change; landing it upstream is a separate, deliberate act.
- Version schemes differ per registry — never copy one SDK's version string to
  another. haskell-sdk in particular is on an unrelated `0.0.x` line.
- A registry package for the target spec release may not be published yet for
  every language. If so, bump what you can and leave the rest ⚠️ in the parity
  doc with a note, rather than forcing a mismatched version.
- Umbrella-scoped only; per-SDK build quirks belong in that SDK's own repo.
