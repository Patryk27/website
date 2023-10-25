fw:

let
  index = {
    name = "index.html";
    path = import ./talks/index.nix fw;
  };

  talk = talkId: {
    name = talkId;

    path = fw.pkgs.linkFarm "talk-${talkId}" [{
      name = "index.html";
      path = import ./talks/talk.nix fw talkId;
    }];
  };

  static = ../content/talks/static;

in
fw.pkgs.linkFarm "talks" (
  [ index ]
  ++ (map talk (builtins.attrNames fw.content.talks))
)
