{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 4)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  description = ''
    This is the last part of the Learning to Fly series in which we're coding a
    simulation of evolution using neural network and genetic algorithm.
  '';

  tags = [
    "ai"
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
