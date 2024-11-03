pkgs:

let
  inherit (pkgs) lib;

  crane = (pkgs.crane.mkLib pkgs).overrideToolchain (
    pkgs.rust-bin.fromRustupToolchainFile ./rust/rust-toolchain
  );

  app = crane.buildPackage {
    pname = "render-feed";
    version = "0.1.0";
    cargoExtraArgs = "-p render-feed";

    src = lib.fileset.toSource {
      root = ./rust;

      fileset = lib.fileset.unions [
        ./rust/Cargo.toml
        ./rust/Cargo.lock
        ./rust/common
        ./rust/render-feed
      ];
    };

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
