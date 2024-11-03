{
  title = "Memory for Nothing: Why Vec&lt;usize&gt; is (probably) a bad idea";
  body = builtins.readFile ./body.html;

  description = ''
    Every now and then one has to index something - and what's better than a
    <code>Vec&lt;usize&gt;</code>?
  '';

  tags = [
    "memory"
    "performance"
    "prices"
    "rust"
  ];

  publishedAt = {
    y = 2024;
    m = 10;
    d = 15;
  };
}
