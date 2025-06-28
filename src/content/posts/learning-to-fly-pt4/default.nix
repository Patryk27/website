{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 4)";
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
    part = "The User Interface";
  };

  publishedAt = {
    y = 2021;
    m = 6;
    d = 10;
  };
}
