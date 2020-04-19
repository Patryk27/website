with import <nixpkgs> {};

let
  deps = import ./deps.nix;

  app = pkgs.rustPlatform.buildRustPackage {
    name = "stdout";
    src = ./.;
    cargoSha256 = "0abpdkqvs8ggw21879ysrmg89bq8riyyqrmcs6n39269gzq9qz08";
    nativeBuildInputs = deps;
  };

in
  stdenv.mkDerivation {
    name = "stdout";

  }