fw:

let
  inherit (fw) pkgs;

  crane = (pkgs.crane.mkLib pkgs).overrideToolchain (
    pkgs.rust-bin.fromRustupToolchainFile ./horizon/rust-toolchain
  );

  horizon = crane.buildPackage {
    pname = "horizon";
    version = "0.1.0";
    src = ./horizon;

    buildInputs = with pkgs; [
      libxml2
      python3Packages.pygments
    ];
  };

in
{
  renderFeed =
    { website }:
    builtins.readFile (
      pkgs.runCommandLocal "render-feed"
        {
          website = builtins.toJSON website;
          passAsFile = [ "website" ];

          buildInputs = with pkgs; [
            libxml2
          ];
        }
        ''
          cat $websitePath | ${horizon}/bin/horizon render-feed > $out
        ''
    );

  renderPost =
    { id, body }:
    builtins.readFile (
      pkgs.runCommandLocal "render-post--${id}"
        {
          inherit body;
          passAsFile = [ "body" ];

          buildInputs = with pkgs; [
            python3Packages.pygments
          ];
        }
        ''
          cat "$bodyPath" | ${horizon}/bin/horizon render-post > $out
        ''
    );
}
