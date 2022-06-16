{
  description = "K8s-CRDs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            overlays = [ rust-overlay.overlay ];
            inherit system;
          };
          lib = pkgs.lib;
          rust = pkgs.rust-bin.stable.latest.default;
          cargoNix = pkgs.callPackage ./Cargo.nix {
            inherit pkgs;
            release = true;
          };
          debugCargoNix = pkgs.callPackage ./Cargo.nix {
            inherit pkgs;
            release = false;
          };
        in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs;
              [
                (rust.override {
                  extensions = [ "rust-src" ];
                })
                cargo-edit
                cargo-watch
                cargo-criterion
                cargo-fuzz
                cargo-flamegraph
                cargo-deny
                cargo-tarpaulin
                crate2nix

                openssl
                pkgconfig

                rnix-lsp
                nixpkgs-fmt
              ];
          };
        });
}
