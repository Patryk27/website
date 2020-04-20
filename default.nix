with import <nixpkgs> {};

let
  cli = import ./cli;

in
  stdenv.mkDerivation {
    name = "stdout";
    phases = [ "buildPhase" ];
    src = ./site;

    buildPhase = ''
      ${cli}/bin/stdout build "$src" "$out"
    '';
  }
