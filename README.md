# Scanza

Scanza is a lightweight Ethereum wallet inspector with a React frontend and an Axum-based Rust backend powered by Alloy.
It allows you to query a wallet's native ETH balance and token holdings.

## Features

- 🦀 Rust backend using [Axum](https://github.com/tokio-rs/axum)
- ⚙️ Ethereum RPC support via [Alloy](https://github.com/alloy-rs/alloy)
- 🧠 Local token metadata loading from disk
- 💡 Modern TypeScript + React frontend built with [Vite](https://vitejs.dev/)
- 🧪 Nix-based development environment
- ⚙️ Zero-config task running via [Just](https://github.com/casey/just)

### API Endpoints

- `GET /wallet/{address}/balance`: Native ETH balance
- `GET /wallet/{address}/tokens`: Top ERC-20 token balances
- `GET /wallet/{address}`: Unified view of ETH and tokens

## Quickstart

### Prerequisites

- [Nix](https://nixos.org/)
- [`direnv`](https://direnv.net/) (optional but recommended)
- `.env` file in project root:
- `.env` file in `web/` directory

`.env`:

```env
RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY
```

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
just cargo run
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
- `clang`, `pkg-config`, `taplo`, `just`, `nodejs`, and `pnpm`
- macOS TLS linker support (via `darwin.apple_sdk.frameworks.Security`)

### Launch dev shell

```bash
nix develop
```
