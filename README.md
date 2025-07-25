# Scanza

Scanza is a lightweight Ethereum wallet inspector. It provides a simple API to fetch an account's native ETH balance and token holdings using an Axum-based Rust backend powered by Alloy.

## Features

- 🦀 Built in Rust using [Axum](https://github.com/tokio-rs/axum)
- ⚙️ Uses [Alloy](https://github.com/alloy-rs/alloy) for Ethereum RPC calls
- 🔍 Supports:
  - `/wallet/{address}/balance`: Native ETH balance
  - `/wallet/{address}/tokens`: ERC-20 token balances (top tokens only)
  - `/wallet/{address}`: Unified wallet view
- 🧠 Local token metadata loading (from Ethereum token list)
- 🧪 Ready-to-hack developer shell with [Nix](https://nixos.org/)

## Quickstart

### Prerequisites

- [Nix](https://nixos.org/)
- `direnv` (optional but recommended)
- `.env` file with your Ethereum RPC:

```env
RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY
```

### Running locally

```bash
direnv allow
just cargo run
```

Visit: [http://localhost:3000/wallet/0xYourAddressHere](http://localhost:3000/wallet/0xYourAddressHere)

## Project Structure

```
src/
├── balance.rs       # ETH balance logic
├── tokens.rs        # ERC-20 token balance logic
├── wallet.rs        # Unified wallet endpoint
├── loader.rs        # Token metadata loader
├── types.rs         # Shared types
├── routes.rs        # Axum route setup
└── main.rs          # Entry point
```

## Development Environment

This project uses a Nix-based dev shell with:

- Nightly Rust (`rust-toolchain.toml`)
- `rust-overlay`
- macOS linker support (if applicable)
- `clang`, `pkg-config`, `taplo` preinstalled

### Dev Shell

```bash
nix develop
```

