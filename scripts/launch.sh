#!/bin/bash

ZOMBIENET_VERSION=v1.3.63

case "$(uname -s)" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          exit 1
esac

if [ $MACHINE = "Linux" ]; then
  ZOMBIENET=zombienet-linux-x64
elif [ $MACHINE = "Mac" ]; then
  ZOMBIENET=zombienet-macos
fi

# Install zombienet
if [ ! -f $ZOMBIENET ]; then
  echo "Fetching zombienet..."
  curl -LO https://github.com/paritytech/zombienet/releases/download/$ZOMBIENET_VERSION/$ZOMBIENET
  chmod +x $ZOMBIENET
fi

# Launch network
echo Launching network...
./$ZOMBIENET spawn -p native network-config.toml