# SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
#
# SPDX-License-Identifier: AGPL-3.0-only

{
  description = "Development environment for my-leptos-app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        pkgsUnstable = import nixpkgs {
          inherit system;
        };
        
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Latest nightly Rust
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-wasip1" "wasm32-unknown-unknown" ];
        };

        # this is how we can tell crane to use our toolchain!
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        # cf. https://crane.dev/API.html#libcleancargosource
        src = craneLib.cleanCargoSource ./.;
        # as before
        nativeBuildInputs = with pkgs; [ rustToolchain pkg-config gcc ];
        buildInputs = with pkgs; [ openssl fontconfig ];
        # because we'll use it for both `cargoArtifacts` and `bin`
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        # remember, `set1 // set2` does a shallow merge:
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
         packages =
            {
              # that way we can build `bin` specifically,
              # but it's also the default.
              inherit bin;
              default = bin;
            };

        devShells.default = pkgs.mkShell {
          # instead of passing `buildInputs` / `nativeBuildInputs`,
            # we refer to an existing derivation here
            inputsFrom = [ bin ];
            buildInputs = with pkgs; [ 
              reuse 
              clippy-sarif 
              sarif-fmt 
              sqlite 
              sqlx-cli 
              cargo-leptos
              pkgsUnstable.fermyon-spin
              gcc
              gnumake
              nodejs_22
              binaryen
              tailwindcss
              direnv
            ];

          # Environment variables
          shellHook = ''
            echo "🦀 Welcome to the my-leptos-app development environment!"
          '';
        };
      }
    );
}
