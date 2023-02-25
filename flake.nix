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
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ] (system:
      let
        name = "k8s-probes-tester";
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) lib;
        rust = pkgs.rust-bin.stable."1.67.0".default;

        craneLib = crane.lib.${system}.overrideToolchain rust;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

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

          # Check Rust formatting
          rustfmt = craneLib.cargoFmt ({
            inherit src;
          });

          # Cargo clippy
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Run tests with cargo-nextest
          test = craneLib.cargoTest (commonArgs // { });

        };

        packages = rec {
          k8s-probes-tester = craneLib.buildPackage (commonArgs // {
            inherit name;
            doCheck = false;
          });
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

