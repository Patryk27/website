fw:

let
  index = {
    name = "index.html";
    path = import ./contact/index.nix fw;
  };

in
fw.pkgs.symlinkJoin {
  name = "contact";

  paths = [
    (fw.pkgs.linkFarm "contact" [ index ])
  ];
}
