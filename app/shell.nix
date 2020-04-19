let
  pkgs = import <nixpkgs> {};
  deps = import ./deps.nix;

in
  pkgs.writeShellScriptBin "testo" ''
    echo yass
  ''
