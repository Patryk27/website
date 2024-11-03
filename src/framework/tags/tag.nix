fw: tag:

let
  objectIds = fw.content.findObjectsByTag tag;

  renderObject =
    obj:
    if obj.type == "post" then
      fw.components.postItem { postId = obj.id; }
    else if obj.type == "talk" then
      fw.components.talkItem { talkId = obj.id; }
    else
      throw "unknown object type: ${obj.type}";

in
fw.components.page {
  title = "~/tags/${tag}";
  layout = "tag";
  withHeader = true;

  body = ''
    <div class="posts talks">
      ${(toString (map renderObject objectIds))}
    </div>
  '';
}
