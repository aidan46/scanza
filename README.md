# Scanza

Scanza is a lightweight Ethereum wallet inspector with a React frontend
and an Axum-based Rust backend powered by Alloy.
It allows you to query a wallet's native ETH balance and token holdings.

## Features

- 🦀 Rust backend using [Axum](https://github.com/tokio-rs/axum)
- ⚙️ Ethereum RPC support via [Alloy](https://github.com/alloy-rs/alloy)
- 🧠 Local token metadata loading from disk
- 💡 Modern TypeScript + React frontend built with [Vite](https://vitejs.dev/)
- 🧪 Nix-based development environment
- ⚙️ Zero-config task running via [Just](https://github.com/casey/just)

### API Endpoints

- `GET /chains`: List of loaded chains
- `GET /{chain}/wallet/{address}/balance`: Native ETH balance
- `GET /{chain}/wallet/{address}/tokens`: Token balances
- `GET /{chain}/wallet/{address}`: Unified view of ETH and tokens

## Quick start

### Prerequisites

- [Nix](https://nixos.org/)
- [`direnv`](https://direnv.net/) (optional but recommended)
- `.env` file in `web/` directory

`web/.env`:

```env
VITE_API_BASE_URL=http://localhost:3000
```

Initialize direnv (optional):

```bash
direnv allow
```

### Running the backend

```bash
cargo run -p server
```

Server runs at: [http://localhost:3000](http://localhost:3000)

### Running the frontend

```bash
just pnpm install
just pnpm run dev
```

App available at: [http://localhost:5173](http://localhost:5173)

You can enter an Ethereum address to inspect its native balance.

## Development Environment

This project includes a fully configured Nix dev shell with:

- Nightly Rust (`rust-toolchain.toml`)
- `rust-overlay` for managing toolchains
- `clang`, `pkg-config`, `taplo`, `just`, `nodejs`, `pre-commit` and `pnpm`
- macOS TLS linker support (via `darwin.apple_sdk.frameworks.Security`)

### Launch dev shell

```bash
nix develop
```
