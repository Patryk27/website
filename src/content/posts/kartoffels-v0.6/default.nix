{
  title = "kartoffels v0.6 released!";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "gamedev"
    "kartoffels"
    "release"
    "rust"
  ];

  publishedAt = {
    y = 2024;
    m = 11;
    d = 20;
  };
}
