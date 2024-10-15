{
  title = "Look Ma: My computer is talking! - Markov chains and N-grams";
  body = builtins.readFile ./body.html;

  description = ''
    ChatGPT is all the hype now, but the math behind it is pretty complex - can
    we create something simpler, possibly under 200 lines of code?
  '';

  tags = [
    "ai"
    "markov-chains"
    "n-grams"
    "rust"
  ];

  publishedAt = {
    y = 2023;
    m = 3;
    d = 23;
  };
}
