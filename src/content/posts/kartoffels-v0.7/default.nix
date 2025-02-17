{
  title = "kartoffels v0.7: Cellular Automata, Statistics, 32-bit RISC-V";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "gamedev"
    "kartoffels"
    "release"
    "rust"
  ];

  publishedAt = {
    y = 2025;
    m = 2;
    d = 17;
  };
}
