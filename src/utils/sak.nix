pkgs:

let
  crane =
    (pkgs.crane.mkLib pkgs).overrideToolchain
      (pkgs.rust-bin.fromRustupToolchainFile ./sak/rust-toolchain);

  app = crane.buildPackage {
    src = ./sak;
    doCheck = true;

    buildInputs = with pkgs; [
      libxml2
      python3Packages.pygments
    ];
  };

  compileFeed = { website }:
    builtins.readFile (
      pkgs.runCommandLocal "compile-feed"
        {
          website = builtins.toJSON website;
          passAsFile = [ "website" ];

          buildInputs = with pkgs; [
            libxml2
          ];
        } ''
        cat $websitePath | ${app}/bin/sak compile-feed > $out
      ''
    );

  compilePost = { id, body }:
    builtins.readFile (
      pkgs.runCommandLocal "compile-post-${id}"
        {
          inherit body;
          passAsFile = [ "body" ];

          buildInputs = with pkgs; [
            python3Packages.pygments
          ];
        } ''
        cat "$bodyPath" | ${app}/bin/sak compile-post > $out
      ''
    );

in
{
  inherit compileFeed compilePost;
}
