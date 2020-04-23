let
  pkgs = import <nixpkgs> {};
  cli-deps = import ./cli/deps.nix;

  src = ./site;
  dst = ./site-build;

  do-build = pkgs.writeShellScriptBin "do-build" ''
    set -e

    rm -rf site.out
    cargo run --manifest-path=cli/Cargo.toml build site site.out
  '';

  do-serve = pkgs.writeShellScriptBin "do-serve" ''
    set -e

    cargo run --manifest-path=cli/Cargo.toml serve site.out
  '';

in
  pkgs.mkShell {
    buildInputs =
        (with pkgs; [ rustup ]) ++
        (with cli-deps; [ asciidoctor sass ]) ++
        [ do-build do-serve ];
  }
