# u5c-factory

Umbrella / wrapper repository for **UTxO RPC** ("u5c") — an RPC interface for
interacting with UTxO-based blockchains.

This repo contains no product code. It aggregates the specification and every
language SDK as pinned git submodules and serves as the entry point for
cross-cutting concerns. See [`AGENTS.md`](./AGENTS.md) for the repository map
and routing rules.

## Layout

| Path               | Repository                                   |
|--------------------|----------------------------------------------|
| `spec/`            | https://github.com/utxorpc/spec              |
| `sdks/rust-sdk/`   | https://github.com/utxorpc/rust-sdk          |
| `sdks/node-sdk/`   | https://github.com/utxorpc/node-sdk          |
| `sdks/go-sdk/`     | https://github.com/utxorpc/go-sdk            |
| `sdks/haskell-sdk/`| https://github.com/utxorpc/haskell-sdk       |
| `sdks/python-sdk/` | https://github.com/utxorpc/python-sdk        |
| `sdks/dotnet-sdk/` | https://github.com/utxorpc/dotnet-sdk        |

## Cloning

```sh
git clone --recurse-submodules https://github.com/utxorpc/u5c-factory.git
```

If you already cloned without submodules:

```sh
git submodule update --init --recursive
```

To pull the latest umbrella state and refresh submodule pointers:

```sh
git pull
git submodule update --init --recursive
```
