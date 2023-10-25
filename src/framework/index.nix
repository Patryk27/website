fw:

let
  inherit (fw.pkgs) lib;

  recentObjects =
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
      (lib.lists.take 5 fw.content.objects);

in
fw.components.page
{
  title = "pwy.io";
  layout = "index";
} ''
  ${fw.content.meta.intro}

  <hr>

  <section class="newest-stuff">
    <h3>Newest stuff</h3>

    <ul>
      ${toString recentObjects}
    </ul>
  </section>

  <hr>

  ${fw.content.meta.contact}
''
