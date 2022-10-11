fw: postId:

let
  renderPost = import ./post/render-post.nix fw;
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
            ${toString (map tag post.tags)}
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

  postIndex = [{
    name = "index.html";

    path =
      let
        vars = {
          "{{ assets }}" = "/posts/${postId}/assets";
        };

        postBody = renderPost {
          id = postId;

          body =
            builtins.replaceStrings
              (builtins.attrNames vars)
              (builtins.attrValues vars)
              (builtins.readFile post.body);
        };

      in
      fw.components.page
        {
          title = post.title;
          layout = "post";
        }
        ''
          ${postHeader}
          ${postSeries}
          ${postBody}
        '';
  }];

  postAssets =
    if post ? assets then
      [{
        name = "assets";
        path = import post.assets fw;
      }]
    else
      [ ];

in
fw.pkgs.linkFarm "post-${postId}" (
  postIndex ++ postAssets
)
