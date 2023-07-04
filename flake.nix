{
  description = "bellado";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    forAllSystems = nixpkgs.lib.genAttrs ["x86_64-linux" "x86_64-darwin" "i686-linux" "aarch64-linux" "aarch64-darwin"];
    pkgs = system: (import nixpkgs {
      inherit system;
      overlays = [
        (import inputs.rust-overlay)
      ];
    });
    toolchain = system: (pkgs system).rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    naersk = system: let
      toolchain' = toolchain system;
    in
      (pkgs system).callPackage inputs.naersk {
        cargo = toolchain';
        rustc = toolchain';
      };
  in rec {
    packages = forAllSystems (system: rec {
      bellado = (pkgs system).callPackage ./. {
        naersk = (naersk system).lib.${system};
      };
      default = bellado;
    });

    # For `nix develop` (optional, can be skipped):
    devShells = forAllSystems (system: rec {
      bellado = (pkgs system).mkShell {
        nativeBuildInputs = [(toolchain system)];
      };
      default = bellado;
    });

    overlays = rec {
      bellado = final: prev: {
        bellado = self.packages.${prev.system}.bellado;
      };
      default = bellado;
    };
  };
}
