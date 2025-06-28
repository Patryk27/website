{
  title = "Learning to Fly series got updated to 2024!";
  body = builtins.readFile ./body.html;

  tags = [
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
