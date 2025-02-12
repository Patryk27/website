pkgs:

let
  crane = (pkgs.crane.mkLib pkgs).overrideToolchain (
    pkgs.rust-bin.fromRustupToolchainFile ./rust/rust-toolchain
  );

  app = crane.buildPackage {
    pname = "render-post";
    version = "0.1.0";

    src = ./rust;
    cargoExtraArgs = "-p render-post";

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
    }
    ''
      cat "$bodyPath" | ${app}/bin/render-post > $out
    ''
)
