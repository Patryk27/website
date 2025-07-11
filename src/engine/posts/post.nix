fw: postId:

let
  post = fw.content.posts.${postId};

  postHeader =
    let
      tag = tag: ''
        <span>#</span><a class="post-meta-tag" href="/tags/${tag}">${tag}</a>
      '';

    in
    ''
      <div class="post-header">
        <h1 class="post-title">
          ${post.title}
        </h1>

        <div class="post-meta">
          ${
            if post ? publishedAt then
              ''
                <div class="post-meta-time">
                  ${fw.components.date post.publishedAt}
                </div>
              ''
            else
              ""
          }
          ${
            if post ? tags then
              ''
                <div class="post-meta-tags">
                  ${toString (map tag (post.tags or [ ]))}
                </div>
              ''
            else
              ""
          }
        </div>
      </div>
    '';

  postSeries' =
    let
      relatedPostIds = builtins.filter (
        postId: fw.content.posts.${postId}.series.id or "" == post.series.id
      ) (builtins.attrNames fw.content.posts);

      relatedPost = postId: ''
        <li>
          <a href="/posts/${postId}">
            ${fw.content.posts.${postId}.series.part}
          </a>
        </li>
      '';

    in
    ''
      <div class="post-series">
        <p>
          This post is part of the <b>${post.series.id}</b> series:
        </p>

        <ol>
          ${toString (map relatedPost relatedPostIds)}
        </ol>
      </div>
    '';

  postSeries = if post ? series then postSeries' else "";

  nextPost =
    if post ? series && post.series ? next then
      let
        nextPostId = post.series.next;
        nextPost = fw.content.posts.${nextPostId};

      in
      ''
        <div class="next-post">
          <a href="/posts/${nextPostId}">
            next post: <b>${nextPost.series.part}</b>
          </a>
        </div>
      ''
    else
      "";

  postIndex = [
    {
      name = "index.html";

      path =
        let
          vars = {
            "{{ assets }}" = "/posts/${postId}/assets";
          };

          postBody = fw.horizon.renderPost {
            id = postId;
            body = builtins.replaceStrings (builtins.attrNames vars) (builtins.attrValues vars) post.body;
          };

        in
        fw.components.page {
          id = "post-${postId}";
          title = post.title;
          layout = "post";

          head = ''
            <meta name="title" content="${post.title}">

            <meta property="og:image" content="https://pwy.io/favicon.png">
            <meta property="og:site_name" content="pwy.io">
            <meta property="og:title" content="${post.title}">
            <meta property="og:type" content="article">
            <meta property="og:url" content="https://pwy.io/posts/${postId}">

            <meta property="twitter:card" content="summary">
            <meta property="twitter:title" content="${post.title}">
          '';

          body = ''
            ${postHeader}
            ${postSeries}
            ${postBody}
            ${nextPost}
          '';
        };
    }
  ];

  postAssets =
    if post ? assets then
      [
        {
          name = "assets";
          path = post.assets;
        }
      ]
    else
      [ ];

in
fw.pkgs.linkFarm "post-${postId}" (postIndex ++ postAssets)
