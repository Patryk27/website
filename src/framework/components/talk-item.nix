fw: { talkId, titleTag ? "h2" }:

let
  talk = fw.content.talks.${talkId};

  renderResource = { label, link }:
    ''<a href="${link}">${label}</a>'';

  renderedResources = builtins.concatStringsSep ", " (
    builtins.map renderResource talk.resources
  );

in
''
  <article class="talk">
    <${titleTag} class="talk-title">
      ${talk.title}
    </${titleTag}>

    ${
      if talk ? subtitle then
        ''
          ${talk.subtitle}
          <br>
        ''
      else
        ""
     }

    (${renderedResources}; ${fw.components.date "%0d.%0m.%y" talk.when}, ${talk.where})
  </article>
''
