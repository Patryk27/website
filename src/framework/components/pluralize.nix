count: noun:

let
  plurals = {
    "post" = "posts";
  };

in
if count == 1 then
  "1 ${noun}"
else
  "${toString count} ${plurals.${noun}}"
