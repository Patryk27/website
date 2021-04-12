{ pkgs }:
let
  inherit (pkgs) lib stdenv;

  image_optim = pkgs.image_optim.override {
    withPngcrush = false;
    withPngout = false;
    withAdvpng = false;
    withOptipng = false;
    withPngquant = false;
    withJhead = false;
    withJpegoptim = false;
    withJpegrecompress = false;
    withJpegtran = false;
    withGifsicle = false;
  };

  buildPage = postId: pageIdx: pageLabel: stdenv.mkDerivation {
    name = "sketches_${postId}_${toString pageIdx}";
    src = ./. + "/${postId}/images.pdf";
    phases = [ "buildPhase" "installPhase" ];

    buildPhase = ''
      mkdir $out
      cd $out

      ${pkgs.inkscape}/bin/inkscape \
          --export-filename=page.svg \
          --pdf-page="${toString pageIdx}" \
          --export-area-drawing \
          $src

      ${image_optim}/bin/image_optim \
          --no-pngcrush \
          --no-pngout \
          --no-advpng \
          --no-optipng \
          --no-pngquant \
          --no-jhead \
          --no-jpegoptim \
          --no-jpegtran \
          --no-gifsicle \
          page.svg
    '';

    installPhase = ''
      mkdir -p resources/${postId}
      mv page.svg resources/${postId}/${pageLabel}.svg
    '';
  };

  buildPost = postId:
    let
      labels = lib.splitString "\n" (
        builtins.readFile (./. + "/${postId}/labels.txt")
      );

    in
    lib.imap1 (buildPage postId) labels;

  posts = builtins.attrNames (
    lib.filterAttrs
      (n: v: v == "directory")
      (builtins.readDir ./.)
  );

in
pkgs.symlinkJoin {
  name = "sketches";
  paths = lib.flatten (builtins.map buildPost posts);
}
