fw:

let
  talkIds =
    builtins.sort
      (a: b:
        fw.utils.dateLessThat
          fw.content.talks.${b}.when
          fw.content.talks.${a}.when)
      (builtins.attrNames fw.content.talks);

  renderTalk = talkId:
    ''
      <li>
        ${
          fw.components.talkItem {
            inherit talkId;
          }
        }
      </li>
    '';

in
fw.components.page
{
  title = "~/talks";
  layout = "talks";
} ''
  <p>
    Source codes & related materials for talks are available
    <a href="https://github.com/Patryk27/talks">at my GitHub repository</a>.
  </p>

  <ul class="talks">
    ${toString (map renderTalk talkIds)}
  </ul>
''
