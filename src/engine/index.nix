fw:

let
  objects =
    let
      render =
        { id, type, ... }:
        if type == "post" then
          (renderPost id)
        else if type == "talk" then
          (renderTalk id)
        else
          throw "unknown object type: ${type}";

      renderPost =
        postId:
        fw.components.postItem {
          inherit postId;
        };

      renderTalk =
        talkId:
        fw.components.talkItem {
          inherit talkId;
        };

    in
    map render fw.content.objects;

in
fw.components.page {
  title = "pwy.io | Patryk Wychowaniec";
  layout = "index";

  body = ''
    ${fw.content.meta.intro}
    ${fw.content.meta.contact}

    <hr>

    <section class="posts talks">
      ${toString objects}
    </section>
  '';
}
