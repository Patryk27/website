let
  pkgs = import <nixpkgs> {};

  asciidoctor = pkgs.writeShellScriptBin "asciidoctor" ''
    asciidoctor="${pkgs.asciidoctor}/bin/asciidoctor"
    find="${pkgs.findutils}/bin/find"

    HOME=/tmp $asciidoctor \
        -s \
        --failure-level=WARN \
        -r asciidoctor-diagram \
        -a source-highlighter=pygments \
        -

    code=$?

    # Since I'm using only `opts=inline`, I don't need no diagram leftovers
    if [[ -n $($find . -maxdepth 1 -name 'diag-*') ]]; then
        rm diag-*
    fi

    exit $code
  '';

  python = pkgs.python2.withPackages (
    ps: with ps; [
      pygments
    ]
  );

in
  with pkgs; [
    asciidoctor
    graphviz
    python
    sass
  ]
