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

  rev = builtins.hashString "sha1" "${style},${pygments},${fonts}";

in
{
  inherit rev;

  drv = pkgs.linkFarm "theme" [
    {
      name = "style.${rev}.css";
      path = style;
    }
    {
      name = "pygments.${rev}.css";
      path = pygments;
    }
    {
      name = "fonts";
      path = fonts;
    }
  ];
}
