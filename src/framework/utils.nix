fw: {
  linkFarmEx = import ./utils/link-farm-ex.nix fw;
  prettifyHtml = import ./utils/prettify-html.nix fw;
  renderFeed = import ../utils/render-feed.nix fw.pkgs;
  renderPost = import ../utils/render-post.nix fw.pkgs;

  dateLessThat = a: b: if a.y == b.y then if a.m == b.m then a.d < b.d else a.m < b.m else a.y < b.y;
}
