fw:
{ postId }:

let
  post = fw.content.posts.${postId};

  tags = toString (
    map (tag: ''
      <span>#</span><a class="post-meta-tag" href="/tags/${tag}">${tag}</a>
    '') post.tags
  );

in
''
  <article class="post">
    <h3 class="post-title">
      <a href="/posts/${postId}">
        ${post.title}
      </a>
    </h3>

    <div class="post-meta">
      <div class="post-meta-time">
        ${fw.components.date post.publishedAt}
      </div>

      ${
        if post ? tags then
          ''
            <div class="post-meta-tags">
              ${tags}
            </div>
          ''
        else
          ""
      }
    </div>
  </article>
''
