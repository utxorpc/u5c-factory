# UTxO RPC — SDK Feature-Parity Matrix

**Status:** living reference, reverse-engineered from the SDK submodules.
What each SDK does today, against the [API surface](./sdk-api-surface.md) and
[CI requirements](./sdk-ci-requirements.md).

> Derived, not authoritative. When a cell disagrees with a submodule, trust
> the submodule and update this file. Re-derive after any `spec/` bump or SDK
> pointer change.

Legend: ✅ implemented & idiomatic · ⚠️ partial / workaround / non-idiomatic ·
❌ missing · — not applicable.
SDK refs: rust-sdk, go-sdk, node-sdk, python-sdk, dotnet-sdk, haskell-sdk.

---

## Spec / proto-gen version per SDK

Umbrella pins `spec/` at **v0.19.0** (`v0.19.0-3-g04b3422`); "current" = the
SDK's proto-gen dependency matches that.

| SDK | Dependency | Version | vs pinned spec (v0.19.0) |
|---|---|---|:--:|
| rust-sdk | `utxorpc-spec` (crate) | 0.19.0 | ✅ current (build-verified) |
| go-sdk | `github.com/utxorpc/go-codegen` | v0.19.0 | ✅ current |
| node-sdk | `@utxorpc/spec` (npm) | 0.18.1 | ⚠️ behind² |
| python-sdk | `utxorpc-spec` (pypi) | 0.19.0 | ⚠️ current, unverified³ |
| dotnet-sdk | `Utxorpc.Spec` (nuget) | 0.19.0-alpha | ⚠️ current, unverified³ |
| haskell-sdk | `utxorpc` (hackage) | ≥0.0.19 <0.0.20 | ⚠️ current, unverified³ ¹ |

¹ haskell `utxorpc` uses independent `0.0.x` numbering; `0.0.19.0` is the
Hackage release matching `spec` v0.19.0.
² `@utxorpc/spec` v0.19.0 not published to npm (npm `latest` is 0.18.1).
³ Manifest bumped to the v0.19.0-line release; not build-verified
(`poetry` / `dotnet` / `cabal` toolchains absent at bump time).

---

## CI conformance vs. mandatory contract

Each SDK's `.github/workflows/` against the
[CI requirements](./sdk-ci-requirements.md). A cell is ✅ only for a CI
workflow (not release/publish-only) running on the mandated trigger; a
build/test that runs only on tag/release is ❌ for the contract.

| SDK | PR trigger | main-push trigger | build job | test job | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ❌ | ❌ | ❌ | ❌ | ❌ |
| go-sdk | ✅ | ✅ | ⚠️ | ✅ | ⚠️ ¹ |
| node-sdk | ❌ | ❌ | ❌ | ❌ | ❌ ² |
| python-sdk | ❌ | ❌ | ❌ | ❌ | ❌ ² |
| dotnet-sdk | ❌ | ❌ | ❌ | ❌ | ❌ ² |
| haskell-sdk | ❌ | ❌ | ❌ | ⚠️ | ❌ ³ |

¹ go-sdk `go-test.yml` runs `go test ./...` on `pull_request` + `push`
(main, tags); no dedicated `go build` step.
² rust-sdk has no workflows; node/python/dotnet have only a
release/publish-triggered workflow (no PR or main-push run).
³ haskell `release.yml` runs `stack test` on `workflow_dispatch` + tag push
only — not on PR or main push.

---

## Query
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| ReadParams | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadUtxos | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| SearchUtxos | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadData | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadTx | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadGenesis | ✅ | ✅ | ✅ | ⚠️ | ✅ | ✅ |
| ReadEraSummary | ✅ | ✅ | ✅ | ⚠️ | ✅ | ✅ |
| ReadState | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

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
| Sync **and** async API | — | ⚠️ (ctx) | ✅ | ✅ | ❌ (async-only) | — (IO) |
| FieldMask support | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ⚠️ |
| Cursor pagination params | ✅ | ✅ | ⚠️ | ✅ | ✅ | ⚠️ |
| Auto-pagination iterator | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Retry / backoff | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Batch submit | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| High-level query helpers | ❌ | ⚠️ (cardano) | ✅ | ❌ | ⚠️ (predicate) | ❌ |

---

## Maintenance

- Re-derive when `spec/` is bumped or any SDK pointer changes; cells are only
  meaningful against the SDK commits this umbrella pins.
- Cells conservative: ✅ only for an exposed, idiomatic public method.
- Normative "should" lives in [`sdk-api-surface.md`](./sdk-api-surface.md) and
  [`sdk-ci-requirements.md`](./sdk-ci-requirements.md); not restated here.
