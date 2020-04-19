with import <nixpkgs> {};

let
  stdout = import ./app;

in
  stdenv.mkDerivation {
    name = "stdout";

    src = ./site;

    phases = [ "buildPhase" ];

    buildPhase = ''
        ${stdout}/bin/stdout_cli build "$src" "$out"
    '';
  }
