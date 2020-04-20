with import <nixpkgs> {};

let
  deps = import ./deps.nix;

in
  pkgs.rustPlatform.buildRustPackage {
    name = "stdout";

    src = nix-gitignore.gitignoreSource [] ./.;
    cargoSha256 = "1k98skj1p4bvkrcxm117a8a0w2fjwwk8iazhrm1vajaljy2106mw";
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
