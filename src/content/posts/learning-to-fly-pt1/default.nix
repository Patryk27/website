{
  title = "Learning to Fly: Let's simulate evolution in Rust (pt 1)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  description = ''
    In this series we'll create a simulation of evolution using neural network
    and genetic algorithm.

    I'm going to introduce you to how a basic neural network and genetic
    algorithm works, then we'll implement both in Rust and compile our
    application to WebAssembly.
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
    part = "The Domain";
    next = "learning-to-fly-pt2";
  };

  publishedAt = {
    y = 2021;
    m = 1;
    d = 4;
  };
}
