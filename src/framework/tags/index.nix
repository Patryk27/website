fw:

let
  renderTag = tag:
    let
      postCount =
        builtins.length
          (fw.content.findPostsByTag tag);

    in
    ''
      <li class="tag">
        <a href="/tags/${tag}">${tag}</a>

        <span class="post-count">
          (${fw.components.pluralize postCount "post"})
        </span>
      </li>
    '';

in
fw.components.page
{
  title = "~/tags";
  layout = "tags";
} ''
  <ul class="tags">
    ${toString (map renderTag fw.content.tags)}
  </ul>
''
