{
  inputs = {
    # TODO upgrading causes Hugo to fail :-((
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };

    nixpkgs-master = {
      url = "github:nixos/nixpkgs";
    };

    shorelark = {
      url = "github:patryk27/shorelark";
    };

    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, nixpkgs-master, shorelark, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        inherit (pkgs) stdenv;

        pkgs = (import nixpkgs) {
          inherit system;
        };

        pkgs-master = (import nixpkgs-master) {
          inherit system;
        };

        website-base = (import ./base/default.nix) {
          inherit pkgs;
        };

        website-projects = (import ./projects/default.nix) {
          inherit system pkgs shorelark;
        };

        website-sketches = (import ./sketches/default.nix) {
          inherit pkgs;
        };

        website = hugoArgs: pkgs.symlinkJoin {
          name = "website";

          paths = [
            (website-base.build hugoArgs)
            website-projects
            website-sketches
          ];
        };

      in
      {
        defaultApp = pkgs.writeShellScriptBin "website" ''
          ${pkgs-master.php80}/bin/php -S localhost:1313 -t ${website "-b http://localhost:1313"}
        '';

        defaultPackage = website "";

        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.writeShellScriptBin "do-refresh-pygments-css" ''
              ${website-base.extraDeps.python}/bin/pygmentize -f html -S monokai -a .pygments |
                  ${pkgs.gnused}/bin/sed 's/.pygments ./.pygments .tok-/g'
            '')

            (pkgs.writeShellScriptBin "do-serve" ''
              cd base
              clear && HUGO_NUMWORKERMULTIPLIER=1 hugo serve
            '')
          ] ++ website-base.deps;
        };
      }
    );
}
