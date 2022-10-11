fw: format: { y, m, d }:

let
  inherit (fw.pkgs) lib;

  toStringPadded = n: width:
    lib.strings.fixedWidthString width "0" (toString n);

  vars = {
    "%y" = toString y;

    "%m" = toString m;
    "%0m" = toStringPadded m 2;

    "%d" = toString d;
    "%0d" = toStringPadded d 2;

    "%M" = builtins.elemAt [
      "January"
      "February"
      "March"
      "April"
      "May"
      "June"
      "July"
      "August"
      "September"
      "October"
      "November"
      "December"
    ]
      (m - 1);
  };

in
builtins.replaceStrings
  (builtins.attrNames vars)
  (builtins.attrValues vars)
  format
