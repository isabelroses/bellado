{
  description = "bellado";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    inherit (nixpkgs.lib) genAttrs systems;
    forAllSystems = genAttrs systems.flakeExposed;
    pkgsFor = forAllSystems (system:
      import nixpkgs {
        inherit system;
        overlays = [self.overlays.default];
      });
  in {
    overlays = rec {
      default = final: prev: {
        bellado = prev.callPackage ./. {naersk = naersk.lib."${prev.system}";};
      };
    };

    packages = forAllSystems (s: let
      pkgs = pkgsFor.${s};
    in rec {
      inherit (pkgs) bellado;
      default = bellado;
    });

    devShells = forAllSystems (s: let
      pkgs = pkgsFor.${s};
    in rec {
      bellado = pkgs.mkShell {
        inputsFrom = [pkgs.bellado];
        buildInputs = with pkgs; [rustc rust-analyzer cargo rustfmt clippy];
      };
      default = bellado;
    });
  };
}
