{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        name = "k8s-probes-tester";

        crossSystem = "aarch64-linux";

        pkgs = import nixpkgs {
          inherit crossSystem localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;
        rust = pkgs.pkgsBuildHost.rust-bin.stable."1.67.0".default.override {
          targets = [ "aarch64-unknown-linux-gnu" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rust;

        crateExpression =
          { openssl
          , libiconv
          , lib
          , pkg-config
          , qemu
          , stdenv
          }: craneLib.buildPackage ({
            inherit name;
            src = craneLib.cleanCargoSource ./.;
            doCheck = false;

            depsBuildBuild = [ qemu ];
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";

            # Tell cargo which target we want to build (so it doesn't default to the build system).
            # We can either set a cargo flag explicitly with a flag or with an environment variable.
            cargoExtraArgs = "--target aarch64-unknown-linux-gnu";
          });

      in
      rec {
        checks = {
          # Check Nix formatting
          nixpkgs-fmt = pkgs.runCommand "check-nixpkgs-fmt" { }
            ''
              echo "checking nix formatting"
              ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./flake.nix} ${./cargoCriterion.nix} ${./filterSources.nix}  ${./filterTestSources.nix}
              touch $out
            '';

        };

        packages = rec {
          k8s-probes-tester = pkgs.callPackage crateExpression { };
          container = pkgs.dockerTools.buildImage {
            inherit name;
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [ k8s-probes-tester ];
              pathsToLink = [ "/bin" ];
            };
            config = { Cmd = [ "/bin/k8s-probes-tester" "--address" "0.0.0.0" "--port" "8080" ]; };
          };
          default = container;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust.override {
              extensions = [ "rust-src" ];
            })
          ];
        };

      }
    );
}

