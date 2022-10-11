fw:

let
  inherit (fw.pkgs) lib;

  postIds =
    lib.lists.take 3
      (builtins.sort
        (a: b:
          fw.utils.dateLessThat
            fw.content.posts.${b}.publishedAt
            fw.content.posts.${a}.publishedAt)
        (builtins.attrNames fw.content.posts));

  posts =
    let
      renderPost = postId: fw.components.postItem {
        inherit postId;
        titleTag = "h4";
      };

    in
    map
      (postId: "<li>${renderPost postId}</li>")
      postIds;

in
fw.components.page
{
  title = "pwy.io";
  layout = "index";
} ''
  <section>
    <p>
      Hi there! 👋
    </p>

    <p>
      My name's Patryk - I'm a software developer who finds joy in throwing
      algorithms at problems and observing what happens; here you can read about
      my adventures.
    </p>
  </section>

  <hr>

  <section>
    <h3>Newest stuff</h3>

    <ul class="posts">
      ${toString posts}
    </ul>
  </section>

  <hr>

  <section>
    <div class="subsection">
      <h3>Want to talk?</h3>

      <ul>
        <li><code>pwychowaniec [@at@] pm.me</code> (if you use GPG, <a href="/pwy.asc">here's my signature</a>)</li>
        <li><a href="https://github.com/patryk27">github.com/patryk27</a></li>
        <li><a href="https://keybase.io/patryk27">keybase.io/patryk27</a></li>
        <li><a href="https://twitter.com/piterolex">twitter.com/piterolex</a></li>
      </ul>
    </div>

    <div class="subsection">
      <h3>Interested in updates?</h3>

      <ul>
        <li>
          <a href="/feed.xml">
            Subscribe to my RSS/Atom feed.
          </a>
        </li>
      </ul>
    </div>

    <div class="subsection">
      <h3>Found a mistake?</h3>

      <ul>
        <li>
          <a href="https://github.com/Patryk27/website">
            Let me know!
          </a>
        </li>
      </ul>
    </div>
  </section>
''
