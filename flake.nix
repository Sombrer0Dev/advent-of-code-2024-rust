{
  description = "Rust AOC24";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        version = "0.1.0";
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "advent-of-code-2024-rust";
            inherit version;
            src = ./.;
            pwd = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };
        apps = {
          solve = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/advent_of_code_2024_rust";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            rust-analyzer

            # pre-commit
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
