# Run the backend server from the root of the project
run-server:
  cargo run --manifest-path server/Cargo.toml

# Run rust tests from the root of the project
rust-test:
  cargo test --manifest-path server/Cargo.toml

