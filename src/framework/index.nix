fw:

let
  objects =
    let
      render = { id, type, ... }:
        if type == "post" then
          (renderPost id)
        else if type == "talk" then
          (renderTalk id)
        else
          throw "unknown object type: ${type}";

      renderPost = postId: fw.components.postItem {
        inherit postId;
        titleTag = "h4";
      };

      renderTalk = talkId: fw.components.talkItem {
        inherit talkId;
        titleTag = "h4";
      };

    in
    map
      (object: "<li>${render object}</li>")
      fw.content.objects;

in
fw.components.page {
  title = "pwy.io | Patryk Wychowaniec";
  layout = "index";

  body = ''
    ${fw.content.meta.intro}
    ${fw.content.meta.contact}

    <hr>

    <section class="posts talks">
      <h3>
        <a href="/posts">Posts</a> & <a href="/talks">Talks</a>
      </h3>

      <ul>
        ${toString objects}
      </ul>
    </section>
  '';
}
