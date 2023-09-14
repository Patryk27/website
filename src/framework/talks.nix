fw:

let
  index = {
    name = "index.html";
    path = import ./talks/index.nix fw;
  };

  static = ../content/talks/static;

in
fw.pkgs.symlinkJoin {
  name = "talks";

  paths = [
    (fw.pkgs.linkFarm "talks" [ index ])
  ] ++ [ static ];
}
