{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball {
        url = "https://github.com/oxalica/rust-overlay/archive/c3e43223dece545cfe06ddd92fd782adc73d56c3.tar.gz";
        sha256 = "sha256-wOmpZis06pVKTR+5meGwhrW10/buf98lnA26uQLaqek=";
      }))
    ];
  },
  lib ? pkgs.lib,
  system ? pkgs.system,
  naersk ? (
    let toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml; in
    pkgs.callPackage (builtins.fetchTarball {
      url = "https://github.com/nix-community/naersk/archive/8507af04eb40c5520bd35d9ce6f9d2342cea5ad1.tar.gz";
      sha256 = "sha256-jXTut7ZSYqLEgm/nTk7TuVL2ExahTip605bLINklAnQ=";
    }) {
      cargo = toolchain;
      rustc = toolchain;
    }
  ),
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
  naersk.buildPackage {
    inherit (manifest) version;
    pname = manifest.name;
    root = lib.cleanSource ./.;
  }
