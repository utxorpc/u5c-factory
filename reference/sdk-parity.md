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
dependency resolves to that.

| SDK | Dependency | Declared | Published SDK | vs pinned spec |
|---|---|---|---|:--:|
| rust-sdk | `utxorpc-spec` (crate) | `0.19.0` | 0.14.0 | ✅ current ¹ |
| go-sdk | `github.com/utxorpc/go-codegen` | `v0.19.2` | v0.1.0 | ✅ current |
| node-sdk | `@utxorpc/spec` (npm) | `^0.19.2` | 0.9.0 | ✅ current |
| python-sdk | `utxorpc-spec` (pypi) | `0.19.2` | 0.2.0 | ✅ current |
| dotnet-sdk | `Utxorpc.Spec` (nuget) | `0.19.2-alpha` | 1.8.0-alpha | ✅ current ² |
| haskell-sdk | `utxorpc` (hackage) | `>=0.0.19 <0.0.20` | 0.0.5.0 | ✅ current ³ |

¹ Written as `"0.19.0"`, which Cargo reads as the caret range `^0.19.0` and
resolves to 0.19.2. Effectively current, though the declared floor is stale.
² `Utxorpc.Spec` ships on NuGet as an `-alpha` prerelease; `0.19.2-alpha`
matches the pinned spec tag.
³ haskell `utxorpc` uses independent `0.0.x` numbering; the `0.0.19` line
matches spec v0.19.x. `stack.yaml` extra-deps pin `utxorpc-0.0.19.2`
exactly. Note `stack.yaml.lock` still records `0.0.18.1` and has not been
regenerated since; stack detects the mismatch and relocks at build time.

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
[pipeline requirements](./sdk-pipeline.md) §2. A stage cell is ✅ only for a
dedicated, gated stage on the release path; ⚠️ for one that happens
incidentally but is not a distinct gate. "registry/auth = spec" is ✅ only
when the registry and the publish mechanism match the `spec` repo's codegen.

| SDK | tag trigger | verify | build | test | publish | release | auth = spec | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ ² | ✅ | ⚠️ |
| go-sdk | ✅ | — ¹ | ⚠️ ¹ | ⚠️ ¹ | ✅ | ✅ | ✅ | ⚠️ |
| node-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ ² | ✅ | ⚠️ |
| python-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ ² | ✅ | ⚠️ |
| dotnet-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ ² | ❌ ³ | ⚠️ |
| haskell-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

¹ go-sdk's `publish.yml` runs `go build ./...` and `go test ./...` as steps
inside the release job rather than as separate gated stages. `verify` is `—`
rather than a gap: a Go module has no manifest version to check a tag
against, so §3's tag/manifest rule is vacuous here.
² The `release` stage (§2.1) is newly mandated; PRs adding it are open —
rust-sdk #40, python-sdk #12, dotnet-sdk #40, node-sdk #65. Releases for the
current published versions were backfilled by hand in the meantime, so every
SDK has a matching GitHub Release today even where the pipeline does not yet
create one.
³ dotnet-sdk publishes via NuGet trusted publishing (OIDC → short-lived key),
while `spec` still pushes `Utxorpc.Spec` with the long-lived
`NUGET_REGISTRY_TOKEN`. The SDK is ahead of both the contract and `spec`
here; §4 needs amending and `spec` migrating, after which this becomes ✅.

Re-derived 2026-08-27 against the post-rewrite pipelines. The API-surface
tables below were **not** re-derived in the same pass and may lag.

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
