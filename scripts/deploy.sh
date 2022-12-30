#!/bin/bash

# See https://docs.moonbeam.network/builders/xcm/xc20/xc20/#calculate-xc20-address for calculating External XC-20 precompile address of xcTRB
xcTRB=0xFFFFFFFF09D483DC7F6434C99FD79C50F2D5EA07
PRIVATE_KEY=0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133

# Deploy contract first, so resulting contract address consistent
echo Deploying Tellor contract...
cd tellor-contracts || exit
# Deploy tellor contract
TELLOR=`forge create --rpc-url http://localhost:9921 --private-key $PRIVATE_KEY --legacy src/Tellor.sol:Tellor | grep "^Deployed to: " | tail -c 43`
if [ -z "$TELLOR" ]
then
  echo Error: Contract could not be deployed.
  exit
fi
echo Tellor contract deployed to "$TELLOR"

# Deploy staking contract
echo Deploying staking contract...
STAKING=`forge create --rpc-url http://localhost:9921 --constructor-args $xcTRB "$TELLOR" --private-key $PRIVATE_KEY --legacy src/Staking.sol:Staking | grep "^Deployed to: " | tail -c 43 || exit`
echo Staking contract deployed to "$STAKING"

# Set staking contract address on tellor contract
echo Registering staking contract with Tellor contract...
cast send --private-key $PRIVATE_KEY --rpc-url http://localhost:9921/ --legacy "$TELLOR" "setStaking(address)" "$STAKING" &> /dev/null || exit

# Deploy governance contract
echo Deploying governance contract...
forge create --rpc-url http://localhost:9921 \
  --constructor-args "$TELLOR" \
  --private-key $PRIVATE_KEY --legacy src/Governance.sol:Governance || exit

cd ..

# Initialise remaining network state
echo Initialising network...
parachains-integration-tests -m test -t network-init.yaml --action-delay 0