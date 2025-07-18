{
  description = "Rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, treefmt-nix, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit overlays;
        system = "x86_64-linux";
      };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs;[
          rustc
          cargo
          rustfmt
          clippy
          bacon
          rust-analyzer
          lld_18
          wasm-bindgen-cli
        ];
      };

      defaultPackage.x86_64-linux =
        pkgs.stdenv.mkDerivation {
        name = "wasm-target";
        src = ./wasm-client;
        buildInputs = [pkgs.lld_18 pkgs.cargo pkgs.rustc pkgs.wasm-bindgen-cli];
        buildPhase = "
            cargo build --target wasm32-unknown-unknown --release
            wasm-bindgen ../target/wasm32-unknown-unknown/release/wasm_client.wasm --out-dir target-wasm
        ";
        installPhase = "mkdir -p $out/bin;";
      };

      formatter.x86_64-linux = treefmt-nix.lib.mkWrapper
        nixpkgs.legacyPackages.x86_64-linux
        {
          projectRootFile = "flake.nix";
          programs.nixpkgs-fmt.enable = true;
          programs.rustfmt.enable = true;
        };
    };
}
