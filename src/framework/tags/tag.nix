fw: tag:

let
  postIds =
    builtins.sort
      (a: b:
        fw.utils.dateLessThat
          fw.content.posts.${b}.publishedAt
          fw.content.posts.${a}.publishedAt)
      (fw.content.findPostsByTag tag);

  renderPost = postId: fw.components.postItem {
    inherit postId;
  };

in
fw.components.page
{
  title = "~/tags/${tag}";
  layout = "tag";
} ''
  <h1>
    Posts tagged #${tag}:
  </h1>

  <div class="posts">
    ${(toString (map renderPost postIds))}
  </div>
''
