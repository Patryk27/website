{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
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

      in
      rec {
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

          render-sketch = utils.lib.mkApp {
            drv = import ./src/utils/render-sketch.nix pkgs;
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
