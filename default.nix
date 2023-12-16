{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "bellado";
  version = "0.3.0";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = false;
}
