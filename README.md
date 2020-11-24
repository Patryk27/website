# pwychowaniec.com

This repository contains source code for [my website](https://pwychowaniec.com).

Powered by [Hugo](https://gohugo.io) and [Nix](https://nixos.org/).

## Building

```shell
$ nix build
```

## Running locally

```shell
$ nix develop
$ cd src
$ HUGO_NUMWORKERMULTIPLIER=1 hugo serve
```

## License

Copyright (c) 2019-2020, Patryk Wychowaniec (`pwychowaniec @at@ pm.me`).    
Licensed under the MIT license.