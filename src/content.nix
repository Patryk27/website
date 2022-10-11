pkgs: {
  index = import ./content/index.nix;
  posts = import ./content/posts.nix pkgs;
  resources = import ./content/resources.nix;
  talks = import ./content/talks.nix;
}
