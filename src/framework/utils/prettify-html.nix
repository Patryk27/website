fw: html:

fw.pkgs.runCommandLocal "prettify"
{
  inherit html;
  passAsFile = [ "html" ];
} ''
  cat $htmlPath \
    | ${fw.pkgs.nodePackages.prettier}/bin/prettier \
      --no-config \
      --parser html \
    > $out
''
