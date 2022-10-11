fw: name: explicitPaths: implicitPaths':

let
  implicitPaths =
    builtins.map
      (path: {
        inherit path;
        name = builtins.baseNameOf path;
      })
      implicitPaths';

in
fw.pkgs.linkFarm name (explicitPaths ++ implicitPaths)
