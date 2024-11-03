fw:
{ talkId }:

let
  talk = fw.content.talks.${talkId};

  resources = builtins.concatStringsSep ", " (
    map ({ label, link }: ''<a href="${link}">${label}</a>'') talk.resources
  );

  tags = toString (
    map (tag: ''
      <a class="talk-meta-tag" href="/tags/${tag}">
        #${tag}
      </a>
    '') talk.tags
  );

in
''
  <article class="talk">
    <h3 class="talk-title">
      ${if talk ? link then ''<a href="${talk.link}">${talk.title}</a>'' else ''${talk.title}''}
    </h3>

    <div class="talk-meta">
      <div class="talk-meta-time">
        ${fw.components.date "%M %d, %y" talk.when} @ ${talk.where}
      </div>

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
