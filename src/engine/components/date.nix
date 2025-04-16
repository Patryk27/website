fw:
{
  y,
  m,
  d,
}:

let
  inherit (fw.pkgs) lib;

  toStringPadded = n: width: lib.strings.fixedWidthString width "0" (toString n);

in
"${toStringPadded d 2}-${toStringPadded m 2}-${toString y}"
