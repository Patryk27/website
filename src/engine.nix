{
  rev,
  pkgs,
  content,
}:

let
  inherit (fw.pkgs) lib;

  fw = {
    inherit pkgs theme;

    content = content // {
      tags = builtins.sort builtins.lessThan (
        lib.lists.unique (lib.lists.flatten (map (obj: obj.tags) fw.content.objects))
      );

      objects =
        let
          mkPost =
            id:
            let
              post = fw.content.posts.${id};

            in
            if post ? publishedAt then
              {
                inherit id;

                type = "post";
                tags = post.tags or [ ];
                date = post.publishedAt;
              }
            else
              null;

          mkTalk =
            id:
            let
              talk = fw.content.talks.${id};

            in
            {
              inherit id;

              type = "talk";
              tags = talk.tags or [ ];
              date = talk.when;
            };

          posts' = map mkPost (builtins.attrNames fw.content.posts);
          posts = builtins.filter (post: post != null) posts';

          talks = map mkTalk (builtins.attrNames fw.content.talks);

        in
        builtins.sort (a: b: fw.utils.dateLt b.date a.date) (posts ++ talks);

      findObjectsByTag = tag: builtins.filter (obj: builtins.elem tag obj.tags) fw.content.objects;
    };

    components = import ./engine/components.nix fw;
    horizon = import ./engine/horizon.nix fw;
    utils = import ./engine/utils.nix fw;
  };

  contact = import ./engine/contact.nix fw;
  feed = import ./engine/feed.nix fw;
  index = import ./engine/index.nix fw;
  posts = import ./engine/posts.nix fw;
  tags = import ./engine/tags.nix fw;
  talks = import ./engine/talks.nix fw;
  theme = import ./engine/theme.nix fw;

in
pkgs.symlinkJoin {
  name = "website";

  paths = [
    (pkgs.linkFarm "website" [
      {
        name = "feed.xml";
        path = feed;
      }
      {
        name = "index.html";
        path = index;
      }
      {
        name = "rev";
        path = pkgs.writeText "rev" rev;
      }
      {
        name = "contact";
        path = contact;
      }
      {
        name = "posts";
        path = posts;
      }
      {
        name = "tags";
        path = tags;
      }
      {
        name = "talks";
        path = talks;
      }
      {
        name = "theme";
        path = theme.drv;
      }
    ])
  ] ++ [ content.static ];
}
