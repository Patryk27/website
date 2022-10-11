fw:

let
  inherit (fw) pkgs;

  style = pkgs.runCommand "style" { } ''
    ${pkgs.sass}/bin/scss \
        --sourcemap=none \
        --style compressed \
        ${./theme}/style.scss \
        $out
  '';

  pygments = pkgs.runCommand "pygments-css" { } ''
    ${pkgs.python3Packages.pygments}/bin/pygmentize \
      -f html \
      -S monokai \
      -a .listing \
      > $out
  '';

  fonts = pkgs.linkFarm "fonts" [
    {
      name = "iosevka.woff2";

      path =
        let
          iosevka = pkgs.iosevka.override {
            set = "custom";

            privateBuildPlan = {
              family = "Iosevka Custom";
              spacing = "term";
              serifs = "sans";
              no-cv-ss = true;
              no-ligation = true;

              metric-override = {
                leading = 1025;
              };

              widths = {
                normal = {
                  menu = 5;
                  css = "normal";
                  shape = 480;
                };
              };

              variants = {
                inherits = "ss08";
              };
            };
          };

        in
        pkgs.runCommand "iosevka" { } ''
          cp ${iosevka}/share/fonts/truetype/iosevka-custom-light.ttf /tmp/font.ttf
          ${pkgs.woff2}/bin/woff2_compress /tmp/font.ttf
          mv /tmp/font.ttf $out
        '';
    }
  ];

in
pkgs.linkFarm "theme" [
  { name = "style.css"; path = style; }
  { name = "pygments.css"; path = pygments; }
  { name = "fonts"; path = fonts; }
]
