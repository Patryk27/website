pkgs: {
  index = import ./content/index.nix;
  posts = import ./content/posts.nix pkgs;
  talks = import ./content/talks.nix;

  static = ./content/static;
}
