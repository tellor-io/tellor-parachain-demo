#!/bin/bash

PIT=parachains-integration-tests#frank/additional-keypair-types
PIT_PACKAGE=@parity/$PIT

# Ensure yarn available
if ! which yarn &> /dev/null
then
echo Error: could not find yarn
exit
fi

# Install parachains-integration-tests
if ! which $PIT &> /dev/null
then
echo Installing parachain integration framework...
yarn global add --silent  --no-progress $PIT_PACKAGE
echo Use \'yarn global remove "$PIT_PACKAGE"\' to remove...
  if ! which $PIT &> /dev/null
  then
    echo Error: could not find \'"$PIT"\' after global install - ensure that the path reported by \'yarn global bin\' is in your PATH.
    exit
  fi
fi

# Launch network
echo Launching network...
parachains-integration-tests -m zombienet -c network-config.toml