#!/bin/bash

ZOMBIENET_VERSION=v1.3.23

# Check if zombienet needs to be downloaded
if [ ! -e zombienet ]; then
  case $(uname) in
    "Darwin") ARCHITECTURE=macos;;
    "Linux")
    {
      # todo: test on linux
      ARCHITECTURE=linux
      if $(uname -m) = x86_64; then ARCHITECTURE=$ARCHITECTURE-x64; else ARCHITECTURE=$ARCHITECTURE-arm64; fi
    } ;;
    esac

  echo Downloading zombienet $ZOMBIENET_VERSION...
  curl -s -L https://github.com/paritytech/zombienet/releases/download/$ZOMBIENET_VERSION/zombienet-$ARCHITECTURE > zombienet
  chmod +x zombienet
fi

echo Launching network...
./zombienet spawn -p native network.toml
