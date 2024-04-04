fw:

let
  renderTag = tag:
    let
      count = type:
        builtins.length
          (builtins.filter
            (obj: obj.type == type)
            (fw.content.findObjectsByTag tag));

      posts = count "post";
      talks = count "talk";

      posts' = fw.components.pluralize "post" posts;
      talks' = fw.components.pluralize "talk" talks;

    in
    ''
      <li class="tag">
        <a href="/tags/${tag}">${tag}</a>

        <span class="counter">
          (${
            if posts == 0 then
              "${talks'}"
            else if talks == 0 then
              "${posts'}"
            else
              "${posts'}, ${talks'}"
          })
        </span>
      </li>
    '';

in
fw.components.page {
  title = "~/tags";
  layout = "tags";
  withHeader = true;

  body = ''
    <ul class="tags">
      ${toString (map renderTag fw.content.tags)}
    </ul>
  '';
}
