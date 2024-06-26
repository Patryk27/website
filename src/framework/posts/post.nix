fw: postId:

let
  post = fw.content.posts.${postId};

  postHeader =
    let
      tag = tag: ''
        <a class="post-meta-tag" href="/tags/${tag}">
          #${tag}
        </a>
      '';

    in
    ''
      <div class="post-header">
        <h1 class="post-title">
          ${post.title}
        </h1>

        <div class="post-meta">
          <time class="post-meta-time">
            ${fw.components.date "%M %d, %y" post.publishedAt}
          </time>

          <div class="post-meta-tags">
            ${toString (map tag (post.tags or []))}
          </div>
        </div>
      </div>
    '';

  postSeries' =
    let
      relatedPostIds =
        builtins.filter
          (postId: fw.content.posts.${postId}.series or "" == post.series)
          (builtins.attrNames fw.content.posts);

      relatedPost = postId:
        ''
          <li>
            <a href="/posts/${postId}">
              ${fw.content.posts.${postId}.subtitle}
            </a>
          </li>
        '';

    in
    ''
      <div class="post-series">
        <p>
          This post is part of the <b>${post.series}</b> series:
        </p>

        <ol>
          ${toString (map relatedPost relatedPostIds)}
        </ol>
      </div>
    '';

  postSeries =
    if post ? series then
      postSeries'
    else
      "";

  nextPost =
    if post ? next then
      let
        nextPostId = post.next;
        nextPost = fw.content.posts.${nextPostId};

      in
      ''
        <div class="next-post">
          <a href="/posts/${nextPostId}">
            next post: <b>${nextPost.subtitle}</b>
          </a>
        </div>
      ''
    else
      "";

  postIndex = [{
    name = "index.html";

    path =
      let
        vars = {
          "{{ assets }}" = "/posts/${postId}/assets";
        };

        postBody = fw.utils.sak.compilePost {
          id = postId;

          body =
            builtins.replaceStrings
              (builtins.attrNames vars)
              (builtins.attrValues vars)
              (builtins.readFile post.body);
        };

      in
      fw.components.page {
        title = post.title;
        layout = "post";

        head = ''
          <meta name="title" content="${post.title}">

          <meta property="og:image" content="https://pwy.io/favicon.png">
          <meta property="og:site_name" content="pwy.io">
          <meta property="og:title" content="${post.title}">
          <meta property="og:type" content="article">
          <meta property="og:url" content="https://pwy.io/posts/${postId}">

          <meta property="twitter:card" content="${post.title}">
          <meta property="twitter:image" content="https://pwy.io/favicon.png">
          <meta property="twitter:title" content="${post.title}">
        '';

        body = ''
          ${postHeader}
          ${postSeries}
          ${postBody}
          ${nextPost}
        '';
      };
  }];

  postAssets =
    if post ? assets then
      [{
        name = "assets";
        path = post.assets;
      }]
    else
      [ ];

in
fw.pkgs.linkFarm "post-${postId}" (
  postIndex ++ postAssets
)
