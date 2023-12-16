self: {
  config,
  lib,
  pkgs,
  ...
}:
with lib; {
  meta.maintainers = [maintainers.isabelroses];

  options.programs.bellado = {
    enable = mkEnableOption "A fast and once simple cli todo tool";

    enableAliases = mkEnableOption "recommended bellado aliases";
  };

  config = let
    cfg = config.programs.bellado;

    aliases =
      {
        bellado = "bellado";
      }
      // optionalAttrs cfg.enableAliases {
        bel = "bellado";
        bell = "bellado -l";
        bella = "bellado -la";
        bellc = "bellado -lc";
      };
  in
    mkIf cfg.enable {
      home.packages = [self.packages.${pkgs.stdenv.hostPlatform.system}.default];

      programs.bash.shellAliases = aliases;

      programs.zsh.shellAliases = aliases;

      programs.fish.shellAliases = aliases;

      programs.ion.shellAliases = aliases;

      programs.nushell.shellAliases = aliases;
    };
}
