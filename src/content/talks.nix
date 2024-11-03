pkgs:

let
  inherit (pkgs) lib;

  talkIds = map (lib.removeSuffix ".nix") (
    builtins.attrNames (
      lib.filterAttrs (entry: entryType: entryType == "regular") (builtins.readDir ./talks)
    )
  );

  mkTalk = talkId: {
    name = talkId;
    value = import (./talks + "/${talkId}.nix");
  };

in
builtins.listToAttrs (map mkTalk talkIds)
