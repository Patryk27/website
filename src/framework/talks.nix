fw:

let
  index = {
    name = "index.html";
    path = import ./talks/index.nix fw;
  };

in
fw.pkgs.linkFarm "talks" [ index ]
