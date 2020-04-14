let
  pkgs = import <nixpkgs> {};

  # We're using a custom variant of `asciidoctor` that supports rendering Graphviz graphs.
  # Huge thanks to https://zipproth.de/cheat-sheets/hugo-asciidoctor/.
  asciidoctor = pkgs.writeShellScriptBin "asciidoctor" ''
    asciidoctor="${pkgs.asciidoctor}/bin/asciidoctor"
    find="${pkgs.findutils}/bin/find"

    $asciidoctor \
        -v \
        -s \
        -r asciidoctor-diagram \
        -a source-highlighter=pygments \
        -

    # Since I'm using `opts=inline`, I don't need no diagram artifacts
    if [[ -n $($find . -name 'diag-*' -maxdepth 1 ) ]]; then
        rm diag-*
    fi
  '';

in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      asciidoctor
      graphviz
      (python2.withPackages (ps: with ps; [ pygments ]))
      sass
    ];
  }
