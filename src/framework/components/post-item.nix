fw: { postId, titleTag ? "h2" }:

let
  post = fw.content.posts.${postId};

  renderTag = tag: ''
    <a class="post-meta-tag" href="/tags/${tag}">
      #${tag}
    </a>
  '';

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

      <div class="post-meta-tags">
        ${toString (map renderTag post.tags)}
      </div>
    </div>

    <div class="post-summary">
      ${builtins.readFile post.summary}
    </div>
  </article>
''
