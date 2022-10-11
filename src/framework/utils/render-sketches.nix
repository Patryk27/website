fw: { src, labels }:

let
  inherit (fw) pkgs;
  inherit (pkgs) lib stdenv;

  # Some of those tools are unfree, but we don't need them anyway:
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

  renderSketch = sketchIdx: sketchLabel:
    pkgs.runCommand "sketch-${toString sketchIdx}--${sketchLabel}" { } ''
      echo "[+] Rendering"

      ${pkgs.inkscape}/bin/inkscape \
          --export-filename=/tmp/sketch.svg \
          --pdf-page="${toString sketchIdx}" \
          --export-area-drawing \
          ${src}

      echo
      echo "[+] Optimizing"

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
          --no-oxipng \
          /tmp/sketch.svg

      mv /tmp/sketch.svg $out
    '';

in
pkgs.linkFarm "sketches" (
  lib.imap1
    (sketchIdx: sketchLabel: {
      name = "${sketchLabel}.svg";
      path = renderSketch sketchIdx sketchLabel;
    })
    labels
)
