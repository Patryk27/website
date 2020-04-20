let
  pkgs = import <nixpkgs> {};

  asciidoctor = pkgs.writeShellScriptBin "asciidoctor" ''
    # Required for `asciidoctor-diagram`
    export PATH="$PATH:${asciidoctor-graphviz}/bin"

    # Required for `pygments`
    export PATH="$PATH:${asciidoctor-python}/bin"

    # Misc other executables; we're not including them in `PATH` to avoid
    # hitting system limit
    asciidoctor="${pkgs.asciidoctor}/bin/asciidoctor"
    find="${pkgs.findutils}/bin/find"

    # Launch `asciidoctor`
    HOME=/tmp "$asciidoctor" \
        -s \
        --failure-level=WARN \
        -r asciidoctor-diagram \
        -a source-highlighter=pygments \
        -

    code=$?

    # Since I'm using `opts=inline`, I don't need no diagram leftovers
    if [[ -n $("$find" . -maxdepth 1 -name 'diag-*') ]]; then
        rm diag-*
    fi

    exit $code
  '';

  asciidoctor-graphviz = pkgs.graphviz;

  asciidoctor-python = pkgs.python2.withPackages (
    ps: with ps; [
      pygments
    ]
  );

in
  {
    asciidoctor = asciidoctor;
    sass = pkgs.sass;
  }
