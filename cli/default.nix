with import <nixpkgs> {};

let
  deps = import ./deps.nix;

in
  pkgs.rustPlatform.buildRustPackage {
    name = "stdout";

    src = nix-gitignore.gitignoreSource [] ./.;
    cargoSha256 = "0lcv7anaq1vicsp71s66bpgpmjvk5az61c2dk4962w1d0axq3zw6";
    buildType = "debug"; # @todo

    # Required for the tests to run
    nativeBuildInputs = with deps; [
      asciidoctor
      sass
    ];

    buildInputs = with pkgs; [
      makeWrapper
    ];

    postInstall = ''
      wrapProgram "$out/bin/stdout" \
        --suffix PATH : "${deps.asciidoctor}/bin" \
        --suffix PATH : "${deps.sass}/bin"
    '';
  }
