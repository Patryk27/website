fw: { title, layout }: body:

let
  title' =
    if layout == "index" then
      title
    else
      "${title} | pwy.io";

  nav =
    let
      items = [
        { path = "/"; title = "~/"; }
        { path = "/posts"; title = "~/posts"; }
        { path = "/tags"; title = "~/tags"; }
        { path = "/talks"; title = "~/talks"; }
      ];

      renderItem = { path, title }:
        ''<li><a href="${path}">${title}</a></li>'';

    in
    ''
      <nav>
        <ul>
          ${toString (map renderItem items)}
        </ul>
      </nav>
    '';

in
fw.utils.prettifyHtml "page-${title}" ''
  <!DOCTYPE html>
  <html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="/theme/pygments.css" media="all">
    <link rel="stylesheet" href="/theme/style.css" media="all">
    <link rel="icon" type="image/png" href="/favicon.png"/>
    <link rel="shortcut icon" type="image/png" href="/favicon.png"/>
    <title>${title'}</title>
  </head>
  <body>
    <div id="container">
      ${nav}

      <main id="${layout}">
        ${body}
      </main>

      <footer>
        <a href="#">⬆ scroll to top</a>
      </footer>
    </div>
  </body>
  </html>
''
