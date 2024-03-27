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
      ${
        if talk ? link then
          ''<a href="${talk.link}">${talk.title}</a>''
        else
          ''${talk.title}''
      }
      <span class="talk-title-camera">🎥</span>
    </${titleTag}>

    <div class="talk-meta">
      <time class="talk-meta-time">
        ${fw.components.date "%M %d, %y" talk.when}
      </time>

      <div class="talk-meta-place">
        ${talk.where}
      </div>

      ${
        if talk ? resources then
          ''
            <div class="talk-meta-resources">
              ${renderedResources}
            </div>
          ''
        else
          ""
      }
    </div>

    ${
      if talk ? summary then
        ''
          <div class="talk-summary">
            ${talk.summary}
          </div>
        ''
      else
        ""
     }
  </article>
''
