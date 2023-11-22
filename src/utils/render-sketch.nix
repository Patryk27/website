pkgs:

let
  inkscape = pkgs.inkscape;

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

in
pkgs.writeShellScriptBin "render-sketch" ''
  input="$1"
  page="$2"
  output="$3"

  if [[ -z "$input" || -z "$page" || -z "$output" ]]; then
      echo "usage:"
      echo "    nix run .#render-sketch -- file.pdf 1 sketch.png"
      exit 0
  fi

  ${inkscape}/bin/inkscape \
      --export-filename=/tmp/sketch.svg \
      --pdf-page=$page \
      --export-area-drawing \
      $input

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
      $output
''
