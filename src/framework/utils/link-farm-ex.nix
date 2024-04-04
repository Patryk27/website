fw: name: explicitPaths: implicitPaths':

let
  implicitPaths =
    map
      (path: {
        inherit path;
        name = builtins.baseNameOf path;
      })
      implicitPaths';

in
fw.pkgs.linkFarm name (explicitPaths ++ implicitPaths)
