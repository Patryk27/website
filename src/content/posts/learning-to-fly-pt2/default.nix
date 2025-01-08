{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 2)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "ai"
    "genetic-algorithm"
    "neural-network"
    "rust"
    "webassembly"
  ];

  series = {
    id = "learning-to-fly";
    part = "The Neural Network";
    next = "learning-to-fly-pt3";
  };

  publishedAt = {
    y = 2021;
    m = 2;
    d = 1;
  };
}
