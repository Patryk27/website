{ rev, pkgs, libs, content }:

let
  inherit (fw.pkgs) lib;

  fw = {
    inherit rev pkgs libs;

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

      objects =
        let
          posts = builtins.map
            (id: {
              inherit id;
              type = "post";
              date = fw.content.posts.${id}.publishedAt;
            })
            (builtins.attrNames fw.content.posts);

          talks = builtins.map
            (id: {
              inherit id;
              type = "talk";
              date = fw.content.talks.${id}.when;
            })
            (builtins.attrNames fw.content.talks);

        in
        (builtins.sort
          (a: b:
            fw.utils.dateLessThat
              b.date
              a.date)
          (posts ++ talks));
    };

    components = import ./framework/components.nix fw;
    utils = import ./framework/utils.nix fw;
  };

  contact = import ./framework/contact.nix fw;
  feed = import ./framework/feed.nix fw;
  index = import ./framework/index.nix fw;
  posts = import ./framework/posts.nix fw;
  tags = import ./framework/tags.nix fw;
  talks = import ./framework/talks.nix fw;
  theme = import ./framework/theme.nix fw;

in
pkgs.symlinkJoin {
  name = "website";

  paths = [
    (pkgs.linkFarm "website" [
      { name = "feed.xml"; path = feed; }
      { name = "index.html"; path = index; }

      { name = "contact"; path = contact; }
      { name = "posts"; path = posts; }
      { name = "tags"; path = tags; }
      { name = "talks"; path = talks; }
      { name = "theme"; path = theme; }
    ])
  ] ++ [ content.static ];
}
