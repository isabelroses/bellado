{
  pkgs ? import <nixpkgs> {},
  lib ? pkgs.lib,
  system ? pkgs.system,
  naersk ? pkgs.callPackage (builtins.fetchTarball {
    url = "https://github.com/nix-community/naersk/archive/8507af04eb40c5520bd35d9ce6f9d2342cea5ad1.tar.gz";
    sha256 = "sha256:0x024pcj1jwnsdx2lkm12q9zclmrsd74xrvghb2a4qjjnsvywx4d";
  }) {},
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
  naersk.buildPackage {
    inherit (manifest) version;
    pname = manifest.name;
    root = lib.cleanSource ./.;
  }
