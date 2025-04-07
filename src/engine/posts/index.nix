fw:

let
  postIds' = builtins.filter (id: fw.content.posts.${id} ? publishedAt) (
    builtins.attrNames fw.content.posts
  );

  postIds = builtins.sort (
    a: b: fw.utils.dateLt fw.content.posts.${b}.publishedAt fw.content.posts.${a}.publishedAt
  ) postIds';

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
