# UTxO RPC — SDK Feature-Parity Matrix

**Status:** living reference, reverse-engineered from the SDK submodules.
What each SDK does today, against the [API surface](./sdk-api-surface.md) and
[pipeline requirements](./sdk-pipeline.md).

> Derived, not authoritative. When a cell disagrees with a submodule, trust
> the submodule and update this file. Re-derive after any `spec/` bump or SDK
> pointer change.

Legend: ✅ implemented & idiomatic · ⚠️ partial / workaround / non-idiomatic ·
❌ missing · — not applicable.
SDK refs: rust-sdk, go-sdk, node-sdk, python-sdk, dotnet-sdk, haskell-sdk.

---

## Spec / proto-gen version per SDK

Umbrella pins `spec/` at **v0.19.2**; "current" = the SDK's proto-gen
dependency matches that.

| SDK | Dependency | Version | vs pinned spec (v0.19.2) |
|---|---|---|:--:|
| rust-sdk | `utxorpc-spec` (crate) | 0.19.0 | ⚠️ behind ¹ |
| go-sdk | `github.com/utxorpc/go-codegen` | v0.19.0 | ⚠️ behind ¹ |
| node-sdk | `@utxorpc/spec` (npm) | 0.19.0 | ⚠️ behind ¹ |
| python-sdk | `utxorpc-spec` (pypi) | 0.19.2 | ✅ current ² |
| dotnet-sdk | `Utxorpc.Spec` (nuget) | 0.19.2-alpha | ✅ current ³ |
| haskell-sdk | `utxorpc` (hackage) | ≥0.0.19 <0.0.20 | ⚠️ behind ⁴ |

¹ Pins the `0.19.0` release — two patch releases behind the pinned spec
(v0.19.2).
² python-sdk's pinned commit is on the `fix/spec-0.19-protobuf` branch, not
`main`; that branch also drops several Query methods — see the Query table.
³ `Utxorpc.Spec` ships on NuGet as an `-alpha` prerelease; `0.19.2-alpha`
matches the pinned spec tag.
⁴ haskell `utxorpc` uses independent `0.0.x` numbering; the `0.0.19` line
matches spec v0.19.x. The `*.cabal` range `>=0.0.19 && <0.0.20` admits it, but
`stack.yaml` extra-deps still pin `utxorpc-0.0.18.1`.

---

## CI conformance vs. mandatory contract

Each SDK's `.github/workflows/` against the CI pipeline contract in
[pipeline requirements](./sdk-pipeline.md) §1. A cell is ✅ only for a CI
workflow (not release/publish-only) running on the mandated trigger; a
build/test that runs only on tag/release is ❌ for the contract.

| SDK | PR trigger | main-push trigger | build job | test job | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ✅ | ✅ | ✅ | ✅ | ✅ |
| go-sdk | ✅ | ✅ | ⚠️ | ✅ | ⚠️ ¹ |
| node-sdk | ✅ | ✅ | ✅ | ✅ | ✅ |
| python-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ² |
| dotnet-sdk | ✅ | ✅ | ✅ | ✅ | ✅ |
| haskell-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ³ |

¹ go-sdk `go-test.yml` runs `go test ./...` on `pull_request` + `push` (main,
tags); there is no dedicated `go build` step — test compilation is the only
build coverage.
² python-sdk `ci.yml` `test` job runs an import smoke check
(`python -c "import utxorpc"`), not the `pytest` suite.
³ haskell-sdk `ci.yml` `test` job runs `stack build --test --no-run-tests` —
test suites compile but are not executed.

---

## Release conformance vs. mandatory contract

Each SDK's release workflow against the release pipeline contract in
[pipeline requirements](./sdk-pipeline.md) §2. A stage cell is ✅ only
for a dedicated, gated stage on the release path; ⚠️ for one that happens
incidentally (e.g. via a `prepublish` hook or a build script) but is not a
distinct gate. "registry/auth = spec" is ✅ only when the registry and the
publish secret name match the `spec` repo's codegen exactly.

| SDK | tag-push trigger | build stage | test stage | publish stage | registry/auth = spec | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ ¹ |
| go-sdk | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ ² |
| node-sdk | ❌ | ⚠️ | ❌ | ✅ | ❌ | ❌ ³ |
| python-sdk | ✅ | ❌ | ❌ | ⚠️ | ❌ | ❌ ⁴ |
| dotnet-sdk | ❌ | ⚠️ | ❌ | ✅ | ⚠️ | ❌ ⁵ |
| haskell-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ ⁶ |

¹ rust-sdk has no release/publish workflow at all; `Cargo.toml` sets
`publish = false`.
² go-sdk `publish.yml` triggers on `v*.*.*` tags and creates a GitHub Release
+ Go-proxy notify, but runs no `go build` / `go test` gate beforehand.
³ node-sdk `publish.yml` triggers on `release: [published]`, not a tag; build
runs only via the `prepublish` hook; publish uses `NODE_AUTH_TOKEN` rather
than the `spec` repo's npm OIDC trusted publishing.
⁴ python-sdk `release.yml` is tag-triggered but broken — it references
`${{ inputs.registry-token }}` in a non-`workflow_call` workflow, so the PyPI
token resolves empty; it also has no checkout, build, or test.
⁵ dotnet-sdk `publish.yml` triggers on `release: [published]`, not a tag;
build runs inside `publish.sh`; the secret is `NUGET_API_KEY`, not the `spec`
repo's `NUGET_REGISTRY_TOKEN`.
⁶ haskell-sdk `release.yml` is the closest — tag-triggered, builds, tests, and
publishes to Hackage with `HACKAGE_REGISTRY_TOKEN` — but the package version
is hand-edited in the `.cabal` files rather than derived from the tag (§2).

---

## Query
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| ReadParams | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadUtxos | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| SearchUtxos | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadData | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadTx | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadGenesis | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| ReadEraSummary | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| ReadState | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

The python-sdk cells reflect the `fix/spec-0.19-protobuf` branch, whose
`QueryClient` currently exposes only `read_params`, `read_utxos`, and
`search_utxos`.

## Submit
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| SubmitTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| EvalTx | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| WaitForTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadMempool | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ |
| WatchMempool | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Sync
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| FetchBlock | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| DumpHistory | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |
| FollowTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Watch
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| WatchTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Cross-cutting capabilities
| Capability | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| TLS config | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Header/metadata auth | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Idiomatic streaming | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ (fold) |
| Sync **and** async API | — | ⚠️ (ctx) | — | ✅ | ❌ (async-only) | — (IO) |
| FieldMask support | ⚠️ | ⚠️ | ❌ | ✅ | ✅ | ⚠️ |
| Cursor pagination params | ✅ | ✅ | ❌ | ✅ | ✅ | ⚠️ |
| Auto-pagination iterator | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Retry / backoff | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Batch submit | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| High-level query helpers | ❌ | ⚠️ (cardano) | ✅ | ❌ | ⚠️ (predicate) | ❌ |

Notes on capability cells:
- rust/go FieldMask is ⚠️: the request type carries it, but the client
  wrappers hardcode `None` (rust) or only use it inside the `cardano` helper
  package (go) — never user-facing on the core Query client.
- node FieldMask and cursor pagination are ❌: neither is surfaced as a
  parameter on the public search/read methods.
- "Sync **and** async API" is `—` for rust/node/haskell, where network I/O is
  async-only by language idiom and a blocking tier is not applicable.

---

## Maintenance

- Re-derive when `spec/` is bumped or any SDK pointer changes; cells are only
  meaningful against the SDK commits this umbrella pins.
- Cells conservative: ✅ only for an exposed, idiomatic public method.
- Normative "should" lives in [`sdk-api-surface.md`](./sdk-api-surface.md) and
  [`sdk-pipeline.md`](./sdk-pipeline.md); not restated here.
</content>
</invoke>
