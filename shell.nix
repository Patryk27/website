with import <nixpkgs> {};

let
  deps = import ./deps.nix;

in
  pkgs.mkShell {
    buildInputs = with deps; [
      asciidoctor
      hugo
      sass
    ];
  }
