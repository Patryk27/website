fw: name: xml:

fw.pkgs.runCommandLocal "prettify-${name}"
{
  inherit xml;
  passAsFile = [ "xml" ];
} ''
  echo "== Input =="
  cat $xmlPath
  echo
  echo "==========="

  cat $xmlPath \
    | ${fw.pkgs.libxml2}/bin/xmllint --format - \
    > $out
''
