fw:

let
  index = {
    name = "index.html";
    path = import ./posts/index.nix fw;
  };

  post = postId: {
    name = postId;
    path = import ./posts/post.nix fw postId;
  };

in
fw.pkgs.linkFarm "posts" ([ index ] ++ (map post (builtins.attrNames fw.content.posts)))
