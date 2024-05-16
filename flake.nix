{
  description = "Decentralized exchanges indexer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { self, flake-utils, nixpkgs, rust-overlay, crane, advisory-db, ... }:
    let
      systems =
        [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
    in flake-utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        inherit (pkgs) lib stdenv;

        rust-toolchain = (pkgs.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml).override {
            extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer" ];
          };

        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [ pkgs.sqlite ] ++ (lib.optional stdenv.isDarwin
            (with pkgs; [
              libiconv
              darwin.apple_sdk.frameworks.SystemConfiguration
            ]));

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        bitcoin-faucet =
          craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in rec {
        checks = {
          inherit bitcoin-faucet;

          bitcoin-faucet-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          bitcoin-faucet-doc =
            craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });

          # Check formatting
          bitcoin-faucet-fmt = craneLib.cargoFmt { inherit src; };

          # Audit dependencies
          bitcoin-faucet-audit =
            craneLib.cargoAudit { inherit src advisory-db; };

          # Audit licenses
          bitcoin-faucet-deny = craneLib.cargoDeny { inherit src; };

          # Run tests with cargo-nextest
          bitcoin-faucet-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          inherit bitcoin-faucet;
          default = packages.bitcoin-faucet;
          docker = pkgs.dockerTools.buildImage {
            name = "ghcr.io/velnbur/bitcoin-faucet";
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "bash + curl";
              paths = [ pkgs.bash pkgs.curl ];
            };
            config = {
              Cmd = [ "${bitcoin-faucet}/bin/bitcoin-faucet" ];

              ExposedPorts = { "8080/tcp" = { }; };
              Volumes = {
                # Daemon data
                "/root/bitcoin-faucet/" = { };
                # Configuration file
                "/etc/bitcoin-faucet/" = { };
              };
            };
          };
        };

        apps = {
          bootstrapper =
            flake-utils.lib.mkApp { drv = packages.bitcoin-faucet; };
          default = apps.bitcoin-faucet;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = [
            # pkgs.ripgrep
          ];
        };
      });
}
