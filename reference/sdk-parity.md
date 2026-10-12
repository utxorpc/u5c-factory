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

Derived against these pins: `spec` `4ef539a` (v0.19.2) · rust-sdk `ba22657` ·
go-sdk `6527e96` · node-sdk `b93adc7` · python-sdk `045186f` · dotnet-sdk
`33a3b93` · haskell-sdk `9b2b7aa`.

---

## Spec / proto-gen version per SDK

Umbrella pins `spec/` at **v0.19.2**; "current" = the SDK's proto-gen
dependency resolves to that. "API version imported" = the UTxO RPC interface
namespace(s) (`v1alpha`, `v1beta`, `v1`) the SDK's own code imports — the
API it actually binds, which the spec package version alone does not show.

| SDK | Dependency | Declared | Published SDK | vs pinned spec | API version imported |
|---|---|---|---|:--:|:--:|
| rust-sdk | `utxorpc-spec` (crate) | `0.19.0` | 0.14.0 | ✅ current ¹ | `v1alpha` |
| go-sdk | `github.com/utxorpc/go-codegen` | `v0.19.2` | v0.1.0 | ✅ current | `v1beta` ⁴ |
| node-sdk | `@utxorpc/spec` (npm) | `^0.19.2` | 0.9.0 | ✅ current | `v1alpha` ⁵ |
| python-sdk | `utxorpc-spec` (pypi) | `0.19.2` | 0.2.0 | ✅ current | `v1alpha` |
| dotnet-sdk | `Utxorpc.Spec` (nuget) | `0.19.2-alpha` | 1.8.0-alpha | ✅ current ² | `v1alpha` |
| haskell-sdk | `utxorpc` (hackage) | `>=0.0.19 <0.0.20` | 0.0.5.0 | ✅ current ³ | `v1alpha` ⁶ |

¹ Written as `"0.19.0"`, which Cargo reads as the caret range `^0.19.0` and
resolves to 0.19.2. Effectively current, though the declared floor is stale.
² `Utxorpc.Spec` ships on NuGet as an `-alpha` prerelease; `0.19.2-alpha`
matches the pinned spec tag.
³ haskell `utxorpc` uses independent `0.0.x` numbering; the `0.0.19` line
matches spec v0.19.x. `stack.yaml` extra-deps pin `utxorpc-0.0.19.2`
exactly. Note `stack.yaml.lock` still records `0.0.18.1` and has not been
regenerated since; stack detects the mismatch and relocks at build time.
⁴ go-sdk's root package (`sdk`), its `cardano` helpers, tests and examples
import only `go-codegen/utxorpc/v1beta`. A legacy mirror package,
`sdk/v1alpha`, still imports `v1alpha`.
⁵ node-sdk imports only the root of `@utxorpc/spec`. At v0.19.2 that root,
the package's single bundled entry point, re-exports `utxorpc/v1alpha/*` and
nothing else, so the published package offers no `v1beta` to import.
⁶ The spec's Haskell codegen at v0.19.2 generates only `V1alpha` modules, so
haskell-sdk cannot bind `v1beta` until `spec` generates it.

rust-sdk, python-sdk and dotnet-sdk could bind `v1beta`: their v0.19.2 spec
packages ship it alongside `v1alpha`. Every SDK except go-sdk is therefore
scored below against the `v1alpha` service definitions, which lack
`ReadState`.

---

## CI conformance vs. mandatory contract

Each SDK's `.github/workflows/` against the CI pipeline contract in
[pipeline requirements](./sdk-pipeline.md) §1. A cell is ✅ only for a CI
workflow (not release/publish-only) running on the mandated trigger; a
build/test that runs only on tag/release is ❌ for the contract. A test job
that runs no tests, or less than the full suite, is ⚠️.

| SDK | PR trigger | main-push trigger | build job | test job | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ¹ |
| go-sdk | ✅ | ✅ | ✅ | ✅ | ✅ ² |
| node-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ³ |
| python-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ⁴ |
| dotnet-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ⁵ |
| haskell-sdk | ✅ | ✅ | ✅ | ⚠️ | ⚠️ ⁶ |

¹ rust-sdk `ci.yml` runs `cargo test -p utxorpc`, a crate with no tests. The
live-node `utxorpc-tests` workspace member is excluded as stale against the
spec, so the job runs zero tests.
² go-sdk `go-test.yml` runs `go build ./...` then `go test ./...` as steps of
one job on `pull_request` + `push` (main, tags); §1 does not require separate
jobs.
³ node-sdk `ci.yml` runs `vitest --passWithNoTests` excluding
`test/index.test.mts`, its only test file (a live-server suite), so the job
runs zero tests.
⁴ python-sdk `ci.yml` `test` job runs an import smoke check
(`python -c "import utxorpc"`); the repo has no `pytest` suite.
⁵ dotnet-sdk `ci.yml` runs `dotnet test --filter "Category=Unit"`. No test
carries that category (all 23 are live-node integration tests), so the job
runs zero tests.
⁶ haskell-sdk `ci.yml` `test` job runs `stack build --test --no-run-tests` —
test suites compile but are not executed.

---

## Release conformance vs. mandatory contract

Each SDK's release workflow against the release pipeline contract in
[pipeline requirements](./sdk-pipeline.md) §2. A stage cell is ✅ only for a
dedicated, gated stage on the release path; ⚠️ for one that happens
incidentally, or that runs less than the full suite. "auth = spec" is ✅ only
when the registry and the publish mechanism match the `spec` repo's codegen.

| SDK | tag trigger | verify | build | test | publish | release ² | auth = spec | Conformant |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| rust-sdk | ✅ | ✅ | ✅ | ⚠️ ⁴ | ✅ | ✅ | ✅ | ⚠️ |
| go-sdk | ✅ | — ¹ | ⚠️ ¹ | ⚠️ ¹ | ✅ | ⚠️ ⁵ | ✅ | ⚠️ |
| node-sdk | ✅ | ✅ | ✅ | ⚠️ ⁴ | ✅ | ✅ | ✅ | ⚠️ |
| python-sdk | ✅ | ✅ | ✅ | ⚠️ ⁴ | ✅ | ✅ | ✅ | ⚠️ |
| dotnet-sdk | ✅ | ✅ | ✅ | ⚠️ ⁴ | ✅ | ✅ | ✅ ³ | ⚠️ |
| haskell-sdk | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ ⁶ | ✅ | ✅ |

¹ go-sdk's `publish.yml` runs `go build ./...` and `go test ./...` as steps
inside the release job rather than as separate gated stages. `verify` is `—`
rather than a gap: a Go module has no manifest version to check a tag
against, so §3's tag/manifest rule is vacuous here.
² The `release` stage (§2.1) is implemented in all six pipelines —
rust-sdk #40, python-sdk #12, dotnet-sdk #40, node-sdk #65, with go-sdk and
haskell-sdk already carrying one. Releases for the versions published before
the stage existed were backfilled by hand. haskell-sdk's v0.0.5.0 was tagged
after its stage existed; the other four SDKs' stages are first exercised on
their next tag.
³ dotnet-sdk publishes via NuGet trusted publishing (OIDC → short-lived key),
matching `spec` `main` since spec#212 and the mechanism §4 records. The
pinned spec tag v0.19.2 predates spec#212 and still pushes with
`NUGET_REGISTRY_TOKEN`; no spec tag carries the OIDC path yet.
⁴ The release test stage runs the same command as CI, so it inherits that
gap (CI footnotes ¹, ³, ⁴, ⁵): zero tests for rust, node and dotnet, an import
smoke check for python. §2 requires the full suite.
⁵ go-sdk's `createRelease` call hardcodes `prerelease: false`, but the
`v*.*.*` trigger also matches pre-release tags, which §2.1 requires to be
marked as such.
⁶ haskell-sdk creates the GitHub Release as a step of its `publish` job, not
as a separate job (§2.1 SHOULD). Hackage auth is not OIDC, so no credential
scope is widened.

---

## Query
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| ReadParams | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadUtxos | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| SearchUtxos | ✅ | ✅ | ⚠️ | ⚠️ | ✅ | ✅ |
| ReadData | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadTx | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| ReadGenesis | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| ReadEraSummary | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| ReadState | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |

- go-sdk gained `ReadState` and `DumpHistory` in go-sdk#236.
- node `SearchUtxos` is ⚠️: the generic search is private; only fixed
  address/asset helpers are public, with no predicate or pagination.
- python `SearchUtxos` is ⚠️: `async_search_utxos` iterates a unary call
  (raising at runtime), and the sync variant may issue the RPC twice.
- `ReadState` is `v1beta`-only, so it is out of reach for every SDK bound to
  `v1alpha`.

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
| FetchBlock | ✅ | ✅ | ✅ | ⚠️ | ✅ | ✅ |
| DumpHistory | ✅ | ✅ | ✅ | ⚠️ | ✅ | ✅ |
| FollowTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

- python `FetchBlock` is ⚠️: it accepts several refs but returns only the
  first block. python `DumpHistory` is ⚠️: it drops `next_token` and cannot
  start from origin.

## Watch
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| WatchTx | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ |

- rust `WatchTx` is ⚠️: an `Idle` event returns `Ok(None)`, which a caller
  cannot tell from end-of-stream, and the event's block is always empty.

## Cross-cutting capabilities
| Capability | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| TLS config | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Header/metadata auth | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Idiomatic streaming | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ (fold) |
| Sync **and** async API | — | ⚠️ (ctx) | — | ⚠️ | ❌ (async-only) | — (IO) |
| FieldMask support | ⚠️ | ✅ | ❌ | ⚠️ | ✅ | ⚠️ |
| Cursor pagination params | ✅ | ✅ | ⚠️ | ⚠️ | ✅ | ⚠️ |
| Auto-pagination iterator | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| Retry / backoff | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Batch submit | ❌ | ❌ | ❌ | ❌ | ⚠️ (loop) | ❌ |
| High-level query helpers | ⚠️ (pattern) | ⚠️ (cardano) | ✅ | ❌ | ⚠️ (predicate) | ❌ |

Notes on capability cells:
- rust FieldMask is ⚠️: the request types carry it, but every client wrapper
  hardcodes `None`; it is reachable only through the raw tonic client.
- go FieldMask is ✅: the core wrappers take the full generated request, and
  the `cardano` helpers expose `WithSearchFieldMask` (go-sdk#239).
- go auto-pagination is ✅: `SearchUtxosPages` and `DumpHistoryPages` return
  `iter.Seq2` and follow `next_token` (go-sdk#239).
- python's streaming methods are async-only, and its async `SearchUtxos` is
  broken, so "Sync **and** async API" is ⚠️. Its FieldMask is accepted on only
  `read_params`, `search_utxos` and `watch_tx`.
- node and python cursor pagination are ⚠️: `DumpHistory` accepts a start
  token and `max_items` but drops `next_token`, and `SearchUtxos` exposes
  neither. node FieldMask is ❌: no public method takes one.
- .NET batch submit is ⚠️: `SubmitTxAsync(Tx[])` loops unary submits
  client-side, since the spec's `SubmitTxRequest` carries a single tx.
- rust's helper is `match_utxos`, which wraps a Cardano pattern into a
  predicate; there are no by-address or by-asset helpers.
- "Sync **and** async API" is `—` for rust/node/haskell, where network I/O is
  async-only by language idiom and a blocking tier is not applicable.

---

## Maintenance

- Re-derive when `spec/` is bumped or any SDK pointer changes; cells are only
  meaningful against the SDK commits this umbrella pins.
- Cells conservative: ✅ only for an exposed, idiomatic public method.
- Normative "should" lives in [`sdk-api-surface.md`](./sdk-api-surface.md) and
  [`sdk-pipeline.md`](./sdk-pipeline.md); not restated here.
