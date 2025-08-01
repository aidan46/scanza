{
  description = "Scanza";

  inputs = {
    # Utilities to simplify multi-platform support
    flake-utils.url = "github:numtide/flake-utils";

    # Nixpkgs repository (we use the unstable channel for latest packages)
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Rust overlay for installing toolchains via rustup-style config
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Apply the Rust overlay
        overlays = [ (import rust-overlay) ];

        # Import nixpkgs with the overlay
        pkgs = import nixpkgs { inherit system overlays; };

        # Use the toolchain defined in rust-toolchain.toml
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = with pkgs; [
          rustToolchain         # Rust compiler and tools from rust-toolchain.toml
          clang                 # Required for building native dependencies (e.g., bindgen)
          pkg-config            # Helps native libraries communicate their compile/link flags
          taplo                 # TOML formatter/linter
          just                  # Task runner (for Justfile commands)

          nodejs                # Node.js (used for front-end development)
          nodePackages.pnpm     # pnpm package manager (used instead of npm/yarn)
          pre-commit            # Git pre-commit hook manager

          # macOS-specific TLS / linking requirements (ignored on Linux)
          darwin.apple_sdk.frameworks.Security
        ];
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          shellHook = ''
            echo "📦 Welcome to Scanza Dev Shell"
          '';
        };
      });
}
