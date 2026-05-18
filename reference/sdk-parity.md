# UTxO RPC — SDK Feature-Parity Matrix

**Status:** living reference, reverse-engineered from the SDK submodules.
Tracks how each SDK measures against the [ideal SDK API
surface](./sdk-api-surface.md). The surface doc answers *"what should an SDK
expose?"*; this doc answers *"what does each SDK expose today?"*.

> Derived, not authoritative. SDK repos define their actual code. When a cell
> disagrees with a submodule, trust the submodule and update this file.
> Re-derive after any `spec/` bump or coordinated SDK release.

Legend: ✅ implemented & idiomatic · ⚠️ partial / workaround / non-idiomatic ·
❌ missing · — not applicable.
SDK refs: rust-sdk, go-sdk, node-sdk, python-sdk, dotnet-sdk, haskell-sdk.

---

## Spec / proto-gen version per SDK

Each SDK depends on generated code from a specific `spec` release. The umbrella
currently pins `spec/` at **v0.19.0** (`v0.19.0-3-g04b3422`); an SDK is
"current" when its proto-gen dependency matches that.

| SDK | Dependency | Version | vs pinned spec (v0.19.0) |
|---|---|---|:--:|
| rust-sdk | `utxorpc-spec` (crate) | 0.19.0 | ✅ current (build-verified) |
| go-sdk | `github.com/utxorpc/go-codegen` | v0.19.0 | ✅ current |
| node-sdk | `@utxorpc/spec` (npm) | 0.18.1 | ⚠️ behind² |
| python-sdk | `utxorpc-spec` (pypi) | 0.19.0 | ⚠️ current, unverified³ |
| dotnet-sdk | `Utxorpc.Spec` (nuget) | 0.19.0-alpha | ⚠️ current, unverified³ |
| haskell-sdk | `utxorpc` (hackage) | ≥0.0.19 <0.0.20 | ⚠️ current, unverified³ ¹ |

¹ haskell-sdk's `utxorpc` package uses an independent `0.0.x` numbering, not
the `spec` tag scheme; `0.0.19.0` is the Hackage release matching `spec`
v0.19.0.
² `@utxorpc/spec` v0.19.0 is **not yet published to npm** (latest is 0.18.1);
node-sdk cannot be bumped until it is. Left intentionally behind.
³ Manifest bumped to the v0.19.0-line release, but not build-verified —
`poetry` / `dotnet` / `cabal` toolchains were unavailable in the bump
environment. Verify via each SDK's own CI/PR before treating as done.

Re-check these from each SDK's manifest (`Cargo.toml`, `go.mod`,
`package.json`, `pyproject.toml`, `.csproj`, `.cabal`) on every re-derivation —
a method can only be parity-complete if the underlying proto-gen exposes it.

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

## Known parity gaps & themes

1. **Spec lag (largely resolved).** After the `bump-sdk-spec` run, rust/go/
   python/dotnet/haskell manifests now target the v0.19.0-line release;
   rust-sdk is build-verified, the rest manifest-bumped but unverified
   (toolchains absent — verify via each SDK's CI). **node-sdk is the only
   remaining laggard**: `@utxorpc/spec` v0.19.0 is not yet on npm, so it
   stays on 0.18.1 until that package is published. `ReadState` (new in
   v1beta) is still exposed by no SDK — the proto-gen is now present in most,
   but the SDKs have not yet wrapped the method; that is now an SDK-code task,
   no longer a dependency-version blocker.
2. **`ReadData` / `ReadTx`** are nearly absent (only go-sdk has both; rust has
   `ReadTx`). Low-cost wins for SDK maintainers.
3. **`EvalTx`** missing in rust/python/.NET/haskell — blocks off-chain
   fee/script workflows in those languages.
4. **`DumpHistory` absent in go-sdk**, **`ReadMempool` only in go/haskell** —
   uneven Sync/Submit coverage.
5. **Universal gaps:** no SDK offers retry/backoff or auto-pagination
   iterators; batch submit only in .NET. Good cross-cutting roadmap items.

---

## Maintenance

- Re-derive when `spec/` is bumped or any SDK pointer is updated. The matrix
  is only meaningful relative to the SDK commits pinned by this umbrella's
  submodules.
- Keep cells conservative: ✅ only for an exposed, idiomatic public method;
  ⚠️ for workarounds (raw proto access, non-idiomatic shape).
- The expected/ideal shape lives in [`sdk-api-surface.md`](./sdk-api-surface.md);
  do not restate normative requirements here.
- Umbrella-scoped; per-SDK design notes belong in that SDK's repo.
