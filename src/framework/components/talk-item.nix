fw: { talkId, titleTag ? "h2" }:

let
  talk = fw.content.talks.${talkId};

  resources = builtins.concatStringsSep ", " (
    map
      ({ label, link }: ''<a href="${link}">${label}</a>'')
      talk.resources
  );

  tags = toString (
    map
      (tag: ''
        <a class="talk-meta-tag" href="/tags/${tag}">
          #${tag}
        </a>
      '')
      talk.tags
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

      ${
        if talk ? tags then
          ''
            <div class="talk-meta-tags">
              ${tags}
            </div>
          ''
        else
          ""
      }

      <div class="talk-meta-place">
        ${talk.where}
      </div>

      ${
        if talk ? resources then
          ''
            <div class="talk-meta-resources">
              ${resources}
            </div>
          ''
        else
          ""
      }
    </div>

    ${
      if talk ? description then
        ''
          <div class="talk-description">
            ${talk.description}
          </div>
        ''
      else
        ""
     }
  </article>
''
