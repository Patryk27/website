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
          default =
            let
              app = pkgs.writeText "app.py" ''
                import http.server

                class Server(http.server.ThreadingHTTPServer):
                    def finish_request(self, request, client_address):
                        self.RequestHandlerClass(
                            request,
                            client_address,
                            self,
                            directory="${defaultPackage}"
                        )

                class Handler(http.server.SimpleHTTPRequestHandler):
                    def end_headers(self):
                        self.send_my_headers()
                        http.server.SimpleHTTPRequestHandler.end_headers(self)

                    def send_my_headers(self):
                        self.send_header("Cache-Control", "no-cache, no-store, must-revalidate")
                        self.send_header("Pragma", "no-cache")
                        self.send_header("Expires", "0")

                http.server.test(
                    ServerClass=Server,
                    HandlerClass=Handler,
                    port=3080
                )
              '';

            in
            utils.lib.mkApp {
              drv = pkgs.writeShellScriptBin "run" ''
                ${pkgs.python3}/bin/python ${app}
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
