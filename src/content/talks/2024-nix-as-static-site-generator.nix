{
  title = "Nix as Static Site Generator for My Blog";

  description = ''
    <p>
      Since a Nix derivation can be anything, from a text file up to a directory
      tree, then what's stopping us from implementing a static site generator
      using Nix? Certainly not the police!
    </p>
  '';

  tags = [
    "cursed"
    "nix"
  ];

  when = {
    y = 2024;
    m = 10;
    d = 26;
  };

  where = "NixCon 2024";
  link = "https://youtu.be/_7wqXN-7ebw?t=6695";

  resources = [
    {
      label = "code";
      link = "https://github.com/Patryk27/website";
    }
  ];
}
