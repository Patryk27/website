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
              imagemagick
              inkscape
              pdftk
            ]
          }"

          "${./script.sh}" "$@"
        '';
      };
    };
}
