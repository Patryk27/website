fw:

let
  generateFeed = import ./feed/generate-feed.nix fw;

  mkObject = { id, type, ... }:
    if type == "post" then
      (mkObjectPost id)
    else if type == "talk" then
      (mkObjectTalk id)
    else
      throw "unknown object type: ${type}";

  mkObjectPost = id:
    let
      post = fw.content.posts.${id};

    in
    {
      type = "post";
      id = id;
      title = post.title;
      summary = builtins.readFile post.summary;
      date = post.publishedAt;
    };

  mkObjectTalk = id:
    let
      talk = fw.content.talks.${id};

    in
    {
      type = "talk";
      id = id;
      title = talk.title;
      subtitle = talk.subtitle or null;
      date = talk.when;
    };

in
fw.utils.prettifyXml "feed.xml" (
  generateFeed {
    objects = builtins.map mkObject fw.content.objects;
  }
)
