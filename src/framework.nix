{
  rev,
  pkgs,
  content,
}:

let
  inherit (fw.pkgs) lib;

  fw = {
    inherit rev pkgs;

    content = content // {
      tags = builtins.sort builtins.lessThan (
        lib.lists.unique (lib.lists.flatten (map (obj: obj.tags) fw.content.objects))
      );

      object =
        type: id:
        if type == "post" then
          {
            inherit id;

            type = "post";
            tags = fw.content.posts.${id}.tags or [ ];
            date = fw.content.posts.${id}.publishedAt;
          }
        else if type == "talk" then
          {
            inherit id;

            type = "talk";
            tags = fw.content.talks.${id}.tags or [ ];
            date = fw.content.talks.${id}.when;
          }
        else
          throw "unknown object type: ${type}";

      objects =
        let
          posts = map (fw.content.object "post") (builtins.attrNames fw.content.posts);

          talks = map (fw.content.object "talk") (builtins.attrNames fw.content.talks);

        in
        builtins.sort (a: b: fw.utils.dateLessThat b.date a.date) (posts ++ talks);

      findObjectsByTag = tag: builtins.filter (obj: builtins.elem tag obj.tags) fw.content.objects;
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
      {
        name = "feed.xml";
        path = feed;
      }
      {
        name = "index.html";
        path = index;
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
        path = theme;
      }
    ])
  ] ++ [ content.static ];
}
