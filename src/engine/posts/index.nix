fw:

let
  postIds = builtins.sort (
    a: b: fw.utils.dateLessThat fw.content.posts.${b}.publishedAt fw.content.posts.${a}.publishedAt
  ) (builtins.attrNames fw.content.posts);

  renderPost =
    postId:
    fw.components.postItem {
      inherit postId;
    };

in
fw.components.page {
  id = "posts";
  title = "~/posts";
  layout = "posts";

  body = ''
    <div class="posts">
      ${(toString (map renderPost postIds))}
    </div>
  '';
}
