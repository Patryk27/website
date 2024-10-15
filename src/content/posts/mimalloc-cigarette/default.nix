{
  title = "Mimalloc Cigarette: Losing one week of my life catching a memory leak";
  body = builtins.readFile ./body.html;

  description = ''
    Memory allocators are great, I love allocating memory! But they can be a
    great source of pain as well - this is a story of how I lost one week
    catching a memory leak in a core Rust application at work.
  '';

  tags = [
    "memory"
    "mimalloc"
    "prices"
    "rust"
  ];

  publishedAt = {
    y = 2024;
    m = 8;
    d = 21;
  };
}
