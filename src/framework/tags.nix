fw:

let
  index = {
    name = "index.html";
    path = import ./tags/index.nix fw;
  };

  tag = tag: {
    name = tag;

    path = fw.pkgs.linkFarm "tag-${tag}" [{
      name = "index.html";
      path = import ./tags/tag.nix fw tag;
    }];
  };

in
fw.pkgs.linkFarm "tags" (
  [ index ]
  ++ (map tag fw.content.tags)
)
