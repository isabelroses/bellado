self: { config, lib, pkgs, ... }: let
  inherit (lib) mkIf mkEnableOption mkOption types escapeShellArgs optionalAttrs;
in {
  meta.maintainers = [ lib.maintainers.isabelroses ];

  options.programs.bellado = {
    enable = mkEnableOption "A fast and once simple cli todo tool";

    enableAliases = mkEnableOption "recommended bellado aliases";

    extraOptions = mkOption {
      type = types.listOf types.str;
      default = [ ];
      example = [ "--header" ];
      description = ''
        Extra command line options passed to bellado.
      '';
    };
  };

  config = let
    cfg = config.programs.bellado;

    args = escapeShellArgs ++ cfg.extraOptions;

    aliases = {
      bellado = "bellado ${args}";
    } // optionalAttrs cfg.enableAliases {
      bel = "bellado";
      bell = "bellado -l";
      bella = "bellado -la";
      bellc = "bellado -lc";
    };
  in mkIf cfg.enable {
    home.packages = [ self.packages.${pkgs.stdenv.hostPlatform.system}.default ];

    programs.bash.shellAliases = aliases;

    programs.zsh.shellAliases = aliases;

    programs.fish.shellAliases = aliases;

    programs.ion.shellAliases = aliases;

    programs.nushell.shellAliases = aliases;
  };
}