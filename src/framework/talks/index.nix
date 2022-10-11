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
    let
      talk = fw.content.talks.${talkId};

      renderResource = { label, link }:
        ''<a href="${link}">${label}</a>'';

      renderedResources = builtins.concatStringsSep ", " (
        builtins.map renderResource talk.resources
      );

      renderedTitle =
        if talk ? subtitle then
          "<b>${talk.title}</b>:<br>${talk.subtitle}"
        else
          "<b>${talk.title}</b>";

    in
    ''
      <li class="talk">
        ${renderedTitle}<br>
        (${renderedResources}; ${fw.components.date "%0d.%0m.%y" talk.when}, ${talk.where})
      </li>
    '';

in
fw.components.page
{
  title = "~/talks";
  layout = "talks";
} ''
  <p>
    Source codes & related materials for all my talks are available
    <a href="https://github.com/Patryk27/talks">at my GitHub repository</a>.
  </p>

  <ul class="talks">
    ${toString (map renderTalk talkIds)}
  </ul>
''
