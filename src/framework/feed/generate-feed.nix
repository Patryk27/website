fw: website:

let
  crane =
    (fw.libs.crane.mkLib fw.pkgs).overrideToolchain
      (fw.pkgs.rust-bin.fromRustupToolchainFile ./generate-feed/rust-toolchain);

  app = crane.buildPackage {
    src = ./generate-feed;
    doCheck = true;
  };

in
builtins.readFile (
  fw.pkgs.runCommandLocal "generate-feed"
  {
    website = builtins.toJSON website;
    passAsFile = [ "website" ];
  } ''
    cat $websitePath | ${app}/bin/generate-feed > $out
  ''
)
