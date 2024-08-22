{
  title = "Imitating specialization with OIBITs";
  body = builtins.readFile ./body.html;

  description = ''
    Why does the code below compile?
  '';

  tags = [
    "rust"
    "tricks"
  ];

  publishedAt = {
    y = 2020;
    m = 10;
    d = 25;
  };

}
