fw:
{
  title,
  layout,
  head ? "",
  body,
}:

let
  inherit (fw.pkgs) lib;

  title' = if layout == "index" then title else "${title} | pwy.io";

  nav =
    let
      items = [
        {
          path = "/";
          title = "~/";
        }
        {
          path = "/posts";
          title = "~/posts";
        }
        {
          path = "/talks";
          title = "~/talks";
        }
        {
          path = "/tags";
          title = "~/tags";
        }
        {
          path = "/contact";
          title = "~/contact";
        }
      ];

      renderItem =
        item:
        let
          class =
            if item.path == "/" then
              # Never show the home button as active, looks ugly
              ""
            else
              (if lib.strings.hasInfix item.title title then "active" else "");

        in
        ''<li class="${class}"><a href="${item.path}">${item.title}</a></li>'';

    in
    ''
      <nav>
        <ul>
          ${toString (map renderItem items)}
        </ul>
      </nav>
    '';

in
fw.utils.prettifyHtml ''
  <!DOCTYPE html>
  <html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <link rel="stylesheet" href="/theme/pygments.${fw.rev}.css" media="all">
    <link rel="stylesheet" href="/theme/style.${fw.rev}.css" media="all">
    <link rel="icon" type="image/png" href="/favicon.png"/>
    <link rel="shortcut icon" type="image/png" href="/favicon.png"/>
    ${head}

    <title>${title'}</title>
  </head>
  <body>
    <div id="container">
      ${nav}

      <main id="${layout}">
        ${body}
      </main>

      <footer>
        <div class="footer-item footer-item-btn">
          <a href="https://pwy.io">
            <img src="/button.png" />
          </a>
        </div>

        <div class="footer-item footer-item-rev">
          <a href="https://github.com/Patryk27/website">
            ${builtins.substring 0 7 fw.rev}
          </a>
        </div>

        <div class="footer-item footer-item-scroll">
          <a href="#">
            scroll to top
          </a>
        </div>
      </footer>
    </div>
  </body>
  </html>
''