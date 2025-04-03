{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };

    # TODO https://gitlab.com/inkscape/inkscape/-/issues/4878
    nixpkgs-inkscape = {
      url = "github:nixos/nixpkgs/nixos-22.11";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";

      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };

    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      nixpkgs-inkscape,
      rust-overlay,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs' = import nixpkgs {
          inherit system;

          overlays = [
            (import rust-overlay)
          ];
        };

        pkgs = pkgs' // {
          inherit crane;
        };

        pkgs-inkscape = import nixpkgs-inkscape {
          inherit system;
        };

      in
      {
        defaultPackage = import ./src/engine.nix {
          inherit pkgs;

          rev = self.rev or "dirty";
          content = import ./src/content.nix pkgs;
        };

        apps = {
          default = utils.lib.mkApp {
            drv =
              let
                python = pkgs.python3.withPackages (ps: [
                  ps.watchdog
                ]);

              in
              pkgs.writeShellScriptBin "app" ''
                ${python}/bin/python ${./src/app.py}
              '';
          };

          pdf2img = utils.lib.mkApp {
            drv = import ./src/utils/pdf2img.nix pkgs-inkscape;
          };
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            python3Packages.pygments
          ];
        };
      }
    );
}
