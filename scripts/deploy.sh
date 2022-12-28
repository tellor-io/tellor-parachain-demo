#!/bin/bash

# Deploy contract first, so resulting contract address consistent
echo Deploying Tellor contracts to Moonbeam...
cd tellor-contracts || exit
# See https://docs.moonbeam.network/builders/xcm/xc20/xc20/#calculate-xc20-address for calculating External XC-20 precompile address of xcTRB
forge create --rpc-url http://localhost:9921 \
  --constructor-args 0xFFFFFFFF09D483DC7F6434C99FD79C50F2D5EA07 \
  --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --legacy src/Staking.sol:Staking || exit
cd ..

# Initialise remaining network state
echo Initialising network...
parachains-integration-tests -m test -t network-init.yaml --action-delay 0