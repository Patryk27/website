fw:

let
  talkIds = builtins.sort (
    a: b: fw.utils.dateLessThat fw.content.talks.${b}.when fw.content.talks.${a}.when
  ) (builtins.attrNames fw.content.talks);

  renderTalk =
    talkId:
    fw.components.talkItem {
      inherit talkId;
    };

in
fw.components.page {
  title = "~/talks";
  layout = "talks";

  body = ''
    <div class="talks">
      ${toString (map renderTalk talkIds)}
    </div>
  '';
}
