{
  description = "Cryptle";

  inputs.enarx.url = github:enarx/enarx/v0.6.1;
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
  inputs.fenix.url = github:nix-community/fenix;
  inputs.flake-utils.url = github:numtide/flake-utils;
  inputs.naersk.inputs.nixpkgs.follows = "nixpkgs";
  inputs.naersk.url = github:nix-community/naersk;
  inputs.nixpkgs.url = github:profianinc/nixpkgs;

  outputs = {
    self,
    enarx,
    fenix,
    flake-utils,
    naersk,
    nixpkgs,
  }: let
    cargo.toml = builtins.fromTOML (builtins.readFile "${self}/Cargo.toml");

    overlay = final: prev: let
      rust = with fenix.packages.${final.system};
        combine [
          stable.rustc
          stable.cargo
          targets.wasm32-wasi.stable.rust-std
        ];

      naersk-lib = naersk.lib.${final.system}.override {
        cargo = rust;
        rustc = rust;
      };
    in {
      cryptle-wasm = naersk-lib.buildPackage {
        src = self;
        CARGO_BUILD_TARGET = "wasm32-wasi";
      };

      cryptle-enarx = final.stdenv.mkDerivation {
        inherit (cargo.toml.package) version;
        pname = cargo.toml.package.name;

        dontUnpack = true;
        installPhase = ''
          mkdir -p $out
          cp ${final.cryptle-wasm}/bin/cryptle.wasm $out/main.wasm
          cp ${self}/Enarx.toml $out/Enarx.toml
        '';
      };
    };
  in
    with flake-utils.lib.system;
      flake-utils.lib.eachSystem [
        aarch64-darwin
        aarch64-linux
        x86_64-darwin
        x86_64-linux
      ] (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [overlay];
        };
      in {
        formatter = pkgs.alejandra;

        packages.cryptle-wasm = pkgs.cryptle-wasm;
        packages.cryptle-enarx = pkgs.cryptle-enarx;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            enarx.packages.${system}.enarx-static
          ];
        };
      });
}
