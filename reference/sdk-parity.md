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

1. **Spec lag.** Rust/Node/Python track `utxorpc-spec` ~v0.18.1 while `spec/`
   is at v0.19.0 → `ReadState` (v1beta) is unimplemented everywhere. Bumping
   spec deps is the single highest-leverage parity action.
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
