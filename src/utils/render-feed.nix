pkgs:

let
  crane = (pkgs.crane.mkLib pkgs).overrideToolchain (
    pkgs.rust-bin.fromRustupToolchainFile ./rust/rust-toolchain
  );

  app = crane.buildPackage {
    pname = "render-feed";
    version = "0.1.0";

    src = ./rust;
    cargoExtraArgs = "-p render-feed";

    buildInputs = with pkgs; [
      libxml2
    ];
  };

in
{ website }:
builtins.readFile (
  pkgs.runCommandLocal "render-feed"
    {
      website = builtins.toJSON website;
      passAsFile = [ "website" ];

      buildInputs = with pkgs; [
        libxml2
      ];
    }
    ''
      cat $websitePath | ${app}/bin/render-feed > $out
    ''
)
