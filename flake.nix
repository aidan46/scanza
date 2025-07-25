{
  description = "Scanza - Minimal Rust project setup";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = with pkgs; [
          rustToolchain
          clang
          pkg-config
          taplo
          just

          # Required for macOS TLS / linker success
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
