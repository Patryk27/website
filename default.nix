with import <nixpkgs> {};

let
  deps = import ./deps.nix;

in
  stdenv.mkDerivation {
    name = "stdout";
    phases = [ "buildPhase" ];
    src = ./src;

    buildInputs = with deps; [
      asciidoctor
      hugo
      rsync
      sass
    ];

    buildPhase = ''
      set -e

      echo '[+] Copying source files'

      mkdir "$out"
      mkdir "$out/src"

      rsync -a "$src/" "$out/src/"
      chmod 777 -R "$out/src"

      echo '[+] Compiling'

      hugo -s "$out/src" --gc --minify

      echo '[+] Cleaning up'

      mv "$out/src/public" "$out-tmp"
      rm -rf "$out"
      mv "$out-tmp" "$out"
    '';
  }
