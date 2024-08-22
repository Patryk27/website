{
  title = "Graphviz in the service of Factorio (pt 1)";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  description = ''
    Graphviz is a software that renders graphs, Factorio is a game in which you
    build and maintain factories; let’s explore how both can complement each
    other.
  '';

  tags = [
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
