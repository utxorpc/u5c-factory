# Vectors

Encoded UTxO RPC messages built from real chain data. SDKs copy these files and decode them
in their tests, which shows each SDK's generated types read the same data.

Modules are ledger-agnostic, while each ledger has its own message types and its own notion
of versions. So each ledger gets a directory here, with its own source data, index and
generator:

| ledger | directory |
|---|---|
| Cardano | [`cardano/`](cardano/README.md) |

Bitcoin and other ledgers have no vectors yet. A new one goes in its own directory, and an
SDK test copies only the directories it covers.
