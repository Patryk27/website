with import <nixpkgs> {};

let
  deps = import ./deps.nix;

in
  pkgs.rustPlatform.buildRustPackage {
    name = "stdout";

    src = nix-gitignore.gitignoreSource [] ./.;
    cargoSha256 = "1k98skj1p4bvkrcxm117a8a0w2fjwwk8iazhrm1vajaljy2106mw";
    buildType = "debug"; # @todo

    buildInputs = with pkgs; [
      makeWrapper
    ];

    # Required for the tests to run
    nativeBuildInputs = with deps; [
      asciidoctor
      sass
    ];

    postInstall = ''
      wrapProgram "$out/bin/stdout_cli" \
        --suffix PATH : "${deps.asciidoctor}/bin"
        --suffix PATH : "${deps.sass}/bin"
    '';
  }
