fw:
{ postId }:

let
  post = fw.content.posts.${postId};

  tags = toString (
    map (tag: ''
      <a class="post-meta-tag" href="/tags/${tag}">
        #${tag}
      </a>
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
        ${fw.components.date "%M %d, %y" post.publishedAt}
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

    <div class="post-description">
      <p>
        ${post.description}
      </p>
    </div>
  </article>
''
