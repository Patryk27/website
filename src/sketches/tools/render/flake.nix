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

    in
    {
      defaultPackage = with pkgs; {
        x86_64-linux = writeShellScriptBin "render" ''
          export PATH="$PATH:${
            lib.strings.makeBinPath [
              (image_optim.override {
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
              })
              inkscape
              pdftk
            ]
          }"

          "${./script.sh}" "$@"
        '';
      };
    };
}
