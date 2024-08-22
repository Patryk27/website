{
  title = "Learning to Fly series got updated to 2024!";
  body = builtins.readFile ./body.html;

  description = ''
    My Learning to Fly series, a beginner-friendly Rust tutorial combining
    evolution, neural network and genetic algorithm, got an upgrade!
  '';

  tags = [
    "ai"
    "genetic-algorithm"
    "neural-network"
    "rust"
    "webassembly"
  ];

  publishedAt = {
    y = 2024;
    m = 4;
    d = 4;
  };
}
