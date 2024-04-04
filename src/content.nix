pkgs: {
  meta = import ./content/meta.nix;
  posts = import ./content/posts.nix pkgs;
  static = ./content/static;
  talks = import ./content/talks.nix pkgs;
}
