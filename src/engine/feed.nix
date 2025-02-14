fw:

let
  mkObject =
    { id, type, ... }:
    if type == "post" then
      (mkPostObject id)
    else if type == "talk" then
      (mkTalkObject id)
    else
      throw "unknown object type: ${type}";

  mkPostObject =
    id:
    let
      post = fw.content.posts.${id};

    in
    {
      type = "post";
      id = id;
      title = post.title;
      date = post.publishedAt;
    };

  mkTalkObject =
    id:
    let
      talk = fw.content.talks.${id};

    in
    {
      type = "talk";
      id = id;
      title = talk.title;
      date = talk.when;
    };

in
fw.pkgs.writeText "feed.xml" (
  fw.horizon.renderFeed {
    website = {
      objects = map mkObject fw.content.objects;
    };
  }
)
