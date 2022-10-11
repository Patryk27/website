pkgs:

let
  inherit (pkgs) lib;

  postIds =
    builtins.attrNames
      (lib.filterAttrs
        (entry: entryType: entryType == "directory")
        (builtins.readDir ./posts));

  mkPost = postId: {
    name = postId;
    value = import (./posts + "/${postId}/default.nix");
  };

in
builtins.listToAttrs (map mkPost postIds)
