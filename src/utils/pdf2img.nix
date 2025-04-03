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
pkgs.writeShellScriptBin "pdf2img" ''
  input="$1"
  page="$2"
  output="$3"

  if [[ -z "$input" || -z "$page" || -z "$output" ]]; then
      echo "invalid usage"
      exit 1
  fi

  ${inkscape}/bin/inkscape \
      --export-filename=$output \
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
