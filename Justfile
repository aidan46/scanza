# Run a cargo command inside the server folder
cargo command:
  cargo {{command}} --manifest-path server/Cargo.toml

# Run a pnpm command inside the web folder with any arguments
pnpm *args:
  @echo Running: pnpm --dir web {{args}}
  pnpm --dir web {{args}}

