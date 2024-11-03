fw: {
  date = import ./components/date.nix fw;
  page = import ./components/page.nix fw;
  pluralize = import ./components/pluralize.nix;
  postItem = import ./components/post-item.nix fw;
  talkItem = import ./components/talk-item.nix fw;
}
