{
  title = "Graphviz in the service of Factorio (pt 1)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "cursed"
    "dot"
    "factorio"
    "graph"
    "graphviz"
  ];

  publishedAt = {
    y = 2020;
    m = 3;
    d = 3;
  };
}
