# Evaluates each devenv.nix and returns the dirs that have languages.rust.enable set.
# Usage: DEVENV_DIRS="$(newline separated dirs)" nix eval --impure --json --file .github/rust-devenvs.nix
let
  dirs = builtins.getEnv "DEVENV_DIRS";
  nixpkgs = builtins.getFlake "nixpkgs";
  lib = nixpkgs.lib;

  # Unwrap lib.mkDefault / lib.mkForce / lib.mkIf
  unwrap =
    value:
    if value ? _type then
      if value._type == "if" then value.condition && unwrap value.content else unwrap value.content
    else
      value;

  rustEnabled =
    dir:
    let
      module = import (../. + "/${dir}/devenv.nix");
      config =
        if builtins.isFunction module then
          module {
            inherit lib;
            pkgs = nixpkgs.legacyPackages.${builtins.currentSystem};
            config = { };
            inputs = { };
          }
        else
          module;
    in
    unwrap (lib.attrByPath [ "languages" "rust" "enable" ] false config) == true;
in
builtins.filter rustEnabled (lib.filter (dir: dir != "") (lib.splitString "\n" dirs))
