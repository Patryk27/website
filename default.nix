with import <nixpkgs> {};

let
  cli = import ./cli;

in
  stdenv.mkDerivation {
    name = "stdout";
    phases = [ "buildPhase" ];
    src = ./site;

    buildPhase = ''
      set -e

      rsync="${rsync}/bin/rsync"
      stdout="${cli}/bin/stdout"

      echo
      echo '-- Building site'
      echo

      "$stdout" build "$src" "$out"

      echo
      echo '-- Copying static resources'
      echo

      "$rsync" -av "$src/static/" "$out"

      echo
      echo '-- Completed'
    '';
  }
