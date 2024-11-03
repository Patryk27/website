{
  title = "Doomé: What we didn’t know we didn’t know";

  description = ''
    <p>
      Together with a friend we've created a game for GitHub's Game Off 2022
      which utilizes software ray-tracing - in this talk we're going through
      our game's internals, describing its most curious & cursed internals.
    </p>
  '';

  tags = [
    "gamedev"
    "rust"
  ];

  when = {
    y = 2022;
    m = 12;
    d = 1;
  };

  where = "Rust Wrocław";
  link = "https://www.youtube.com/watch?v=S85Tw0dVtmw&t=5306s";

  resources = [
    {
      label = "game";
      link = "https://dzejkop.itch.io/doome";
    }
    {
      label = "code";
      link = "https://github.com/patryk27/doome";
    }
  ];
}
