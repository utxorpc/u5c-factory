# UTxO RPC — Ideal SDK API Surface & Parity Reference

**Status:** living reference, reverse-engineered from the spec and existing SDKs.
**Source of truth for methods:** the `spec/` submodule (protobuf). This document
describes the *ideal, language-agnostic surface* an SDK should expose and tracks
how each SDK currently measures against it.

> This artifact is **derived**, not authoritative. The `spec/` proto definitions
> define the protocol; SDK repos define their actual code. When they disagree,
> trust the submodules and update this file. Re-derive after any spec bump or
> coordinated SDK release (see [bumping skills](../AGENTS.md#skills)).

---

## 1. Scope & method set (from `spec/`, v1beta)

An SDK is "complete" when it exposes all of the following, grouped into four
service clients. Streaming methods are server-streaming (unary request → stream
of responses).

### Query — read-only ledger/chain state
| Method | Kind | Purpose |
|---|---|---|
| `ReadParams` | unary | Current protocol parameters |
| `ReadUtxos` | unary | UTxOs by `TxoRef` (tx hash + index) |
| `SearchUtxos` | unary, paginated | UTxOs matching a predicate |
| `ReadData` | unary | Datums by hash |
| `ReadTx` | unary | Transaction by hash |
| `ReadGenesis` | unary | Genesis config |
| `ReadEraSummary` | unary | Per-era summaries / params |
| `ReadState` | unary | Chain-specific ledger-state queries *(v1beta-only, new)* |

### Submit — transaction lifecycle
| Method | Kind | Purpose |
|---|---|---|
| `SubmitTx` | unary | Submit signed tx |
| `EvalTx` | unary | Evaluate tx (scripts/costs) without submitting |
| `WaitForTx` | **stream** | Tx stage updates (ACKNOWLEDGED→MEMPOOL→NETWORK→CONFIRMED) |
| `ReadMempool` | unary | Point-in-time mempool snapshot |
| `WatchMempool` | **stream** | Mempool changes matching a predicate |

### Sync — chain synchronization
| Method | Kind | Purpose |
|---|---|---|
| `FetchBlock` | unary | Blocks by reference |
| `DumpHistory` | unary, paginated | Block history range |
| `FollowTip` | **stream** | Tip updates with apply / undo / reset (rollback) |
| `ReadTip` | unary | Current chain tip |

### Watch — filtered tx events
| Method | Kind | Purpose |
|---|---|---|
| `WatchTx` | **stream** | Txs matching predicate, with block intersection |

---

## 2. Cross-cutting API conventions (the "ideal" shape)

Derived from where the SDKs already agree; these are the recommended norms.

- **Client construction.** Per-service clients (`QueryClient`, `SubmitClient`,
  `SyncClient`, `WatchClient`) built from `{ uri, headers?, tls?, ... }`.
  Builder (Rust), options object (Go/Node), constructor (Python/.NET), or
  config helper (Haskell) — all acceptable; the *inputs* should match.
- **Auth.** Pass-through metadata/headers map (e.g. `dmtr-api-key`). No bespoke
  bearer/OAuth abstractions — auth is a gRPC-metadata concern, per spec.
- **TLS.** Auto-derived from URI scheme (`https://`) or an explicit flag;
  always available.
- **Streaming.** Exposed as the language's idiomatic async iteration
  (async iterator / `IAsyncEnumerable` / generator / fold). `FollowTip` and
  `WatchTx` carry an **action enum**: `APPLY` / `UNDO` / `RESET`.
- **Pagination.** Cursor-based: `max_items` + `start_token` → `next_token`,
  for `SearchUtxos` and `DumpHistory`.
- **Field selection.** Optional protobuf `FieldMask` accepted on Query / Sync /
  Watch requests.
- **Errors.** Typed transport vs. gRPC vs. parse errors distinguishable.

### Recommended but not yet universal
Retry/backoff, connection pooling, automatic pagination iterators, batch
submit, and high-level query helpers (by address / asset / payment part) — see
matrix. These are *aspirational* parity targets, not spec requirements.

---

## 3. Feature-parity matrix

Legend: ✅ implemented & idiomatic · ⚠️ partial / workaround / non-idiomatic ·
❌ missing · — not applicable.
SDK refs: rust-sdk, go-sdk, node-sdk, python-sdk, dotnet-sdk, haskell-sdk.

### Query
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

### Submit
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| SubmitTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| EvalTx | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| WaitForTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadMempool | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ |
| WatchMempool | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

### Sync
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| FetchBlock | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| DumpHistory | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |
| FollowTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| ReadTip | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

### Watch
| Method | Rust | Go | Node | Python | .NET | Haskell |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| WatchTx | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

### Cross-cutting capabilities
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

## 4. Known parity gaps & themes

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

## 5. Maintenance

- Re-run the derivation when `spec/` is bumped or any SDK pointer is updated.
- Keep cell values conservative: mark ✅ only for an exposed, idiomatic public
  method; use ⚠️ for workarounds (raw proto access, non-idiomatic shape).
- This is umbrella-scoped; per-SDK design notes belong in that SDK's repo.
