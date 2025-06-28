{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 3)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "genetic-algorithm"
    "neural-network"
    "rust"
    "webassembly"
  ];

  series = {
    id = "learning-to-fly";
    part = "The Genetic Algorithm";
    next = "learning-to-fly-pt4";
  };

  publishedAt = {
    y = 2021;
    m = 3;
    d = 24;
  };
}
