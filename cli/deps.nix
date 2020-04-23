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
        "$@" \
        -

    code=$?

    # Clean-up trash
    rm -rf .asciidoctor
    rm -f diag-*

    exit $code
  '';

  asciidoctor-graphviz = pkgs.graphviz;

  asciidoctor-python = pkgs.python2.withPackages (
    ps: with ps; [
      pygments
    ]
  );

  sass = pkgs.writeShellScriptBin "sass" ''
    sass="${pkgs.sass}/bin/sass"

    HOME=/tmp "$sass" "$@"
  '';

in
  {
    inherit asciidoctor;
    inherit sass;
  }
