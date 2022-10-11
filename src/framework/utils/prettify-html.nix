fw: name: html:

fw.pkgs.runCommandLocal "prettify-${name}"
{
  inherit html;
  passAsFile = [ "html" ];
} ''
  echo "== Input =="
  cat $htmlPath
  echo "==========="

  cat $htmlPath \
    | ${fw.pkgs.nodePackages.prettier}/bin/prettier \
      --no-config \
      --parser html \
    > $out
''
