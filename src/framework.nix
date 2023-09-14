{ rev, pkgs, linuxPkgs, libs, content }:

let
  inherit (fw.pkgs) lib;

  fw = {
    inherit rev pkgs linuxPkgs libs;

    content = content // {
      tags =
        builtins.sort
          builtins.lessThan
          (lib.lists.unique
            (lib.lists.flatten
              (map
                (postId: content.posts.${postId}.tags)
                (builtins.attrNames content.posts))));

      findPostsByTag = tag:
        (builtins.filter
          (postId: builtins.elem tag content.posts.${postId}.tags)
          (builtins.attrNames content.posts));
    };

    components = import ./framework/components.nix fw;
    utils = import ./framework/utils.nix fw;
  };

  index = content.index fw;
  feed = import ./framework/feed.nix fw;
  posts = import ./framework/posts.nix fw;
  tags = import ./framework/tags.nix fw;
  talks = import ./framework/talks.nix fw;
  theme = import ./framework/theme.nix fw;

in
pkgs.symlinkJoin {
  name = "website";

  paths = [
    (pkgs.linkFarm "website" [
      { name = "index.html"; path = index; }
      { name = "feed.xml"; path = feed; }
      { name = "posts"; path = posts; }
      { name = "tags"; path = tags; }
      { name = "talks"; path = talks; }
      { name = "theme"; path = theme; }
    ])
  ] ++ [ content.static ];
}
