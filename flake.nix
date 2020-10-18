{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = (import nixpkgs) {
        system = "x86_64-linux";
      };

      asciidoctor = pkgs.writeShellScriptBin "asciidoctor" ''
        # Required for `asciidoctor-diagram`
        export PATH="$PATH:${asciidoctor-graphviz}/bin"

        # Required for `pygments`
        export PATH="$PATH:${asciidoctor-python}/bin"

        # Other executables that don't have to be in `PATH` (to avoid hitting system limit)
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

      pygments-css = pkgs.writeShellScriptBin "pygments-css" ''
        PATH="$PATH:${asciidoctor-python}/bin"
        PATH="$PATH:${pkgs.gnused}/bin"

        pygmentize -f html -S lovelace -a .pygments | sed 's/.pygments ./.pygments .tok-/g'
      '';

      sass = pkgs.writeShellScriptBin "sass" ''
        sass="${pkgs.sass}/bin/sass"

        HOME=/tmp "$sass" "$@"
      '';

    in
    {
      defaultPackage = {
        x86_64-linux = pkgs.stdenv.mkDerivation {
          name = "pwychowaniec.com";
          src = ./src;

          buildInputs = [
            asciidoctor
            pkgs.hugo
            pkgs.rsync
            sass
          ];

          phases = [ "buildPhase" ];

          buildPhase = ''
            set -e

            echo '[+] Copying source files'

            mkdir "$out"
            mkdir "$out/src"

            rsync -a "$src/" "$out/src/"
            chmod 777 -R "$out/src"

            echo '[+] Compiling'

            HUGO_NUMWORKERMULTIPLIER=1 hugo -s "$out/src" --gc --minify

            echo '[+] Cleaning up'

            mv "$out/src/public" "$out-tmp"
            rm -rf "$out"
            mv "$out-tmp" "$out"
          '';
        };
      };

      devShell = {
        x86_64-linux = pkgs.mkShell {
          buildInputs = [
            asciidoctor
            pkgs.hugo
            pkgs.rsync
            sass
          ];
        };
      };
    };
}
