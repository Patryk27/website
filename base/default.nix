{ pkgs }:
let
  inherit (pkgs) stdenv;

  python = pkgs.python2.withPackages (
    ps: with ps; [
      pygments
    ]
  );

  asciidoctor = pkgs.writeShellScriptBin "asciidoctor" ''
    # Required for `asciidoctor-diagram`
    export PATH="$PATH:${pkgs.graphviz}/bin"

    # Required for `pygments`
    export PATH="$PATH:${python}/bin"

    HOME=/tmp ${pkgs.asciidoctor}/bin/asciidoctor \
        -s \
        --failure-level=WARN \
        -r asciidoctor-diagram \
        -a source-highlighter=pygments \
        -

    code=$?

    rm -rf .asciidoctor
    rm -f diag-*

    exit $code
  '';

  sass = pkgs.writeShellScriptBin "sass" ''
    HOME=/tmp ${pkgs.sass}/bin/sass "$@"
  '';

in
rec {
  build = hugoArgs: stdenv.mkDerivation {
    name = "base";
    src = ./.;
    phases = [ "buildPhase" ];
    nativeBuildInputs = deps;

    buildPhase = ''
      # Hugo assumes read-write access to the source code, so we've gotta
      # copy website from the store into somewhere writable
      mkdir /tmp/src
      cp -ar $src/* /tmp/src
      chmod 777 -R /tmp/src

      HUGO_NUMWORKERMULTIPLIER=1 hugo -s /tmp/src -d $out ${hugoArgs}
    '';
  };

  deps = [
    asciidoctor
    pkgs.hugo
    sass
  ];

  extraDeps = {
    inherit python;
  };
}
