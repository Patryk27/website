pkgs:

let
  inherit (pkgs) lib;

  crane =
    (pkgs.crane.mkLib pkgs).overrideToolchain
      (pkgs.rust-bin.fromRustupToolchainFile ./rust/rust-toolchain);

  app = crane.buildPackage {
    pname = "render-post";
    version = "0.1.0";
    cargoExtraArgs = "-p render-post";

    src = lib.fileset.toSource {
      root = ./rust;

      fileset = lib.fileset.unions [
        ./rust/Cargo.toml
        ./rust/Cargo.lock
        ./rust/common
        ./rust/render-post
      ];
    };

    buildInputs = with pkgs; [
      python3Packages.pygments
    ];
  };

in
{ id, body }:
builtins.readFile (
  pkgs.runCommandLocal "render-post--${id}"
  {
    inherit body;
    passAsFile = [ "body" ];

    buildInputs = with pkgs; [
      python3Packages.pygments
    ];
  } ''
    cat "$bodyPath" | ${app}/bin/render-post > $out
  ''
)
