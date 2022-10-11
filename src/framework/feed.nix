fw:

let
  generateFeed = import ./feed/generate-feed.nix fw;

  postIds =
    builtins.sort
      (a: b:
        fw.utils.dateLessThat
          fw.content.posts.${b}.publishedAt
          fw.content.posts.${a}.publishedAt)
      (builtins.attrNames fw.content.posts);

  mkPost = postId:
    let
      post = fw.content.posts.${postId};

    in
    {
      id = postId;
      title = post.title;
      summary = builtins.readFile post.summary;
      publishedAt = post.publishedAt;
    };

in
fw.utils.prettifyXml "feed.xml" (
  generateFeed {
    posts = builtins.map mkPost postIds;
  }
)
