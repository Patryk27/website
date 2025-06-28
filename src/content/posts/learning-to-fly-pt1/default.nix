{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 1)";
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
    part = "The Domain";
    next = "learning-to-fly-pt2";
  };

  publishedAt = {
    y = 2021;
    m = 1;
    d = 4;
  };
}
