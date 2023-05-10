#!/bin/bash

PIT=https://github.com/paritytech/parachains-integration-tests.git#frank/additional-keypair-types
PIT_PACKAGE=parachains-integration-tests

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
  yarn global add $PIT
  echo Use \'yarn global remove "$PIT_PACKAGE"\' to remove...
  if ! which $PIT_PACKAGE &> /dev/null
  then
    echo Error: could not find \'"$PIT_PACKAGE"\' after global install - ensure that the path reported by \'yarn global bin\' is in your PATH.
    exit
  fi
fi

# Launch network
echo Launching network...
parachains-integration-tests -m zombienet -c network-config.toml
