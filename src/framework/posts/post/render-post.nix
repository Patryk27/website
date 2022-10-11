fw: { id, body }:

let
  crane =
    (fw.libs.crane.mkLib fw.pkgs).overrideToolchain
      (fw.pkgs.rust-bin.fromRustupToolchainFile ./render-post/rust-toolchain);

  app = crane.buildPackage {
    src = ./render-post;
    doCheck = true;

    buildInputs = with fw.pkgs; [
      python3Packages.pygments
    ];
  };

in
builtins.readFile (
  fw.pkgs.runCommandLocal "render-post-${id}"
  {
    inherit body;

    passAsFile = [ "body" ];

    buildInputs = with fw.pkgs; [
      python3Packages.pygments
    ];
  } ''
    cat "$bodyPath" | ${app}/bin/render-post > $out
  ''
)
