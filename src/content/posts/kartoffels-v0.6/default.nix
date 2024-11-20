{
  title = "kartoffels v0.6 released!";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  description = ''
    kartoffels is a game where you're given a potato and your job is to
    implement a firmware for it - let's see what changed in v0.6!
  '';

  tags = [
    "devlog"
    "game"
    "kartoffels"
    "rust"
  ];

  publishedAt = {
    y = 2024;
    m = 11;
    d = 20;
  };
}
