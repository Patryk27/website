fw: { postId, titleTag ? "h2" }:

let
  post = fw.content.posts.${postId};

  tags = toString (
    map
      (tag: ''
        <a class="post-meta-tag" href="/tags/${tag}">
          #${tag}
        </a>
      '')
      post.tags
  );

in
''
  <article class="post">
    <${titleTag} class="post-title">
      <a href="/posts/${postId}">
        ${post.title}
      </a>
    </${titleTag}>

    <div class="post-meta">
      <time class="post-meta-time">
        ${fw.components.date "%M %d, %y" post.publishedAt}
      </time>

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

    <div class="post-summary">
      ${builtins.readFile post.summary}
    </div>
  </article>
''
