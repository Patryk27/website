{ system, pkgs, shorelark }:
let
  inherit (pkgs) stdenv;

  shorelark' = stdenv.mkDerivation {
    name = "shorelark";
    src = shorelark.defaultPackage."${system}";
    phases = [ "buildPhase" ];

    buildPhase = ''
      mkdir -p $out/en/projects
      ln -s $src $out/en/projects/shorelark
    '';
  };

in
pkgs.symlinkJoin {
  name = "projects";

  paths = [
    shorelark'
  ];
}
