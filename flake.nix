{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix2container.url = "github:cameronraysmith/nix2container/185-skopeo-fix";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    devshell.url = "github:numtide/devshell";
    nix-oci.url = "github:dauliac/nix-oci";
    crane.url = "github:ipetkov/crane";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, lib, ... }:
        let
          wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
            src = pkgs.fetchCrate {
              pname = "wasm-bindgen-cli";
              version = "0.2.106";
              hash = "sha256-M6WuGl7EruNopHZbqBpucu4RWz44/MSdv6f0zkYw+44=";
            };

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              inherit (src) pname version;
              hash = "sha256-ElDatyOwdKwHg3bNH/1pcxKI7LXkhsotlDPQjiLHBwA=";
            };
          };

          devPackages = with pkgs; [
            dioxus-cli
            wasm-bindgen-cli
            tailwindcss_4
            binaryen
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
          ];

          rustToolchainFor = p:
            p.rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            };
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchainFor;

          unfilteredRoot = ./.;
          src = lib.fileset.toSource {
            root = unfilteredRoot;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources unfilteredRoot)
              (lib.fileset.fileFilter (file: file.hasExt "css") unfilteredRoot)
              (lib.fileset.maybeMissing ./ui)
            ];
          };
          commonArgs = {
            inherit src;
            strictDeps = true;
            doCheck = false;

            nativeBuildInputs = devPackages ++ lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.sigtool
            ];

            buildInputs = [

            ] ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];
          };
          ui_server = craneLib.buildPackage (
            commonArgs
            // {
              pname = "ui";
              version = "0.0.1";
              cargoArtifacts = null;
              # cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
              #   buildPhaseCargoCommand = ''
              #     cargo build --release -p ui --no-default-features --target wasm32-unknown-unknown --features web --locked
              #     cargo build --release -p ui --no-default-features --features server --locked
              #   '';
              # });
              buildPhaseCargoCommand = ''
                DX_HOME=$(mktemp -d) dx bundle --release -p ui
              '';
              doNotPostBuildInstallCargoBinaries = true;
              installPhaseCommand = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/web $out/bin
              '';
              meta.mainProgram = "ui";
            }
          );

        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
              (final: prev: { })
            ];
          };

          devshells.default = {
            packages = devPackages ++ (with pkgs; [
              craneLib.cargo
              craneLib.rustc
              craneLib.clippy
              craneLib.rustfmt
            ]);
          };

          packages.ui = ui_server;

          # oci.containers.default = {
          #   package = self'.packages.ui;
          # };
        };
      flake = { };
    };
}
