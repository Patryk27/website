let
  pkgs = import <nixpkgs> {};
  deps = import ./deps.nix;

in
  pkgs.mkShell {
    buildInputs =
        (with pkgs; [ rustup ]) ++
        (with deps; [ asciidoctor sass ]);
  }
