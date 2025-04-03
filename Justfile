run:
    nix run

fmt:
    nixfmt .

[no-cd]
pdf2img src page dst:
    nix run .#pdf2img {{ src }} {{ page }} {{ dst }}
