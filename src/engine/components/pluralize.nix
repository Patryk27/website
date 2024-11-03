noun: count:

let
  plurals = {
    "post" = "posts";
    "talk" = "talks";
  };

in
if count == 1 then "1 ${noun}" else "${toString count} ${plurals.${noun}}"
