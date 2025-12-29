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
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    flake-root.url = "github:srid/flake-root";
    dioxus.url = "github:simmsb/dioxus";
    dioxus.inputs.flake-parts.follows = "flake-parts";
    dioxus.inputs.nixpkgs.follows = "nixpkgs";
    dioxus.inputs.rust-overlay.follows = "rust-overlay";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
        inputs.nix-oci.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.process-compose-flake.flakeModule
        inputs.flake-root.flakeModule
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

          dioxus-cli = inputs.dioxus.packages.${system}.dioxus-cli;

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
              (lib.fileset.maybeMissing ./server.crt)
              (lib.fileset.maybeMissing ./server.key)
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
              cargoArtifacts = null;
              # cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
              #   buildPhaseCargoCommand = ''
              #     cargo build --release -p ui --no-default-features --target wasm32-unknown-unknown --features web --locked
              #     cargo build --release -p ui --no-default-features --features server --locked
              #   '';
              # });
              buildPhaseCargoCommand = ''
                DX_HOME=$(mktemp -d) dx bundle --web --release -p ui
              '';
              doNotPostBuildInstallCargoBinaries = true;
              installPhaseCommand = ''
                mkdir -p $out
                cp -r target/dx/$pname/release/web $out/bin
              '';
              meta.mainProgram = "ui";
            }
          );

          api_server = craneLib.buildPackage (
            commonArgs
            // {
              pname = "server";
              cargoExtraArgs = "-p kenwood-chef-api";
              meta.mainProgram = "kenwood-chef-api";
            }
          );

          caddyWithConfig = pkgs.writeShellApplication {
            name = "caddy-with-config";
            text = let config = pkgs.writeText "caddyfile" ''
            0.0.0.0:8080 {
              uri strip_prefix {header.X-External-Path}

              reverse_proxy localhost:8181 {
                header_up Accept-Encoding identity

                resp_body_replace "href=\"" "href=\"{http.request.header.X-External-Path}"
                resp_body_replace "src=\"" "src=\"{http.request.header.X-External-Path}"
                resp_body_replace "action=\"" "action=\"{http.request.header.X-External-Path}"
              }
            }
            '';
            in "${lib.getExe pkgs.caddy} run --adapter caddyfile --config ${config}";
          };

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

            env = [
              { name = "FLAKE_ROOT"; eval = "$(${lib.getExe config.flake-root.package})"; }
              { name = "DATABASE_URL"; eval = "sqlite://$FLAKE_ROOT/db.sqlite?mode=rwc"; }
            ];
          };

          packages = {
            ui = ui_server;
            api = api_server;
          };

          process-compose.all = {
            cli = {
              environment.PC_DISABLE_TUI = true;
              options = {
                no-server = true;
              };
            };
            settings.environment = {
              DATABASE_URL = "sqlite:///config/db.sqlite?mode=rwc";
            };
            settings.processes = {
              db_init.command = ''
                ${pkgs.busybox}/bin/mkdir -p $(echo "$DATABASE_URL" | ${pkgs.busybox}/bin/sed 's|.*:///||; s|/[^/]*$||')
                echo | ${lib.getExe pkgs.sqlite} $DATABASE_URL 
              '';
              ui.command = "${lib.getExe config.packages.ui}";
              ui.depends_on."db_init".condition = "process_completed_successfully";
              ui.environment = {
                IP = "0.0.0.0";
                PORT = "8080";
              };
              api.command = "${lib.getExe config.packages.api} server";
              api.depends_on."db_init".condition = "process_completed_successfully";
              # caddy.command = "${lib.getExe caddyWithConfig}";
            };
          };

          oci.containers.default = {
            dependencies = [
              pkgs.sqlite
              pkgs.busybox
            ];
            package = { version = config.packages.api.version; } // pkgs.writeShellApplication {
              name = "kenwood-api";
              text = "${lib.getExe config.packages.all} up";
            };
            registry = "ghcr.io/simmsb";
            push = true;
            isRoot = true;
          };
        };
      oci.enabled = true;
      flake = { };
    };
}
