{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "bellado";
  version = "1.4.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
