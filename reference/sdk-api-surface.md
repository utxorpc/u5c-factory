# UTxO RPC — Ideal SDK API Surface

**Status:** living reference, derived from the protocol.
**Source of truth for methods:** the `spec/` submodule (protobuf).

This document describes the *ideal, language-agnostic surface* an SDK should
expose. It is **normative-by-derivation**: it follows from the protocol, not
from any SDK's implementation. For how each SDK currently measures against it,
see the separate [SDK parity matrix](./sdk-parity.md).

> Derived, not authoritative. The `spec/` proto definitions define the
> protocol. When this doc disagrees with `spec/`, trust the submodule and
> update this file. Re-derive after any spec bump.

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
  Builder, options object, constructor, or config helper — all acceptable; the
  *inputs* should match.
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
submit, and high-level query helpers (by address / asset / payment part).
These are *aspirational* targets, not spec requirements; current adoption is
tracked in the [parity matrix](./sdk-parity.md).

---

## 3. Maintenance

- This is the **normative shape** — change it only when the protocol changes.
  Re-derive after any `spec/` bump.
- Implementation status of these methods/conventions lives in
  [`sdk-parity.md`](./sdk-parity.md), not here. Keep the two separate: this
  doc answers "what should an SDK expose?", the matrix answers "what does each
  SDK expose today?".
- This artifact stays in the wrapper (not `spec/`): it is cross-cutting and
  the umbrella is its owner. Promoting it into `spec/` as a formal conformance
  document is only warranted if `spec` maintainers choose to own it as a
  requirement.
- Umbrella-scoped; per-SDK design notes belong in that SDK's repo.
