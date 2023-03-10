#!/bin/bash

# See https://docs.moonbeam.network/builders/xcm/xc20/xc20/#calculate-xc20-address for calculating External XC-20 precompile address of xcTRB
xcTRB=0xFFFFFFFF09D483DC7F6434C99FD79C50F2D5EA07
PRIVATE_KEY=0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133
MULTISIG=0x0000000000000000000000000000000000000000

# Deploy parachain registry contract first, so resulting contract address consistent
echo Deploying Parachain Registry contract...
cd tellor-contracts || exit
# Deploy tellor parachain registry contract
REGISTRY=`forge create --rpc-url http://localhost:9921 --private-key $PRIVATE_KEY --legacy src/ParachainRegistry.sol:ParachainRegistry | grep "^Deployed to: " | tail -c 43`
if [ -z "$REGISTRY" ]
then
  echo Error: Contract could not be deployed.
  exit
fi
echo Parachain Registry contract deployed to "$REGISTRY"

# Deploy staking contract
echo Deploying staking contract...
STAKING=`forge create --rpc-url http://localhost:9921 --constructor-args "$REGISTRY" $xcTRB --private-key $PRIVATE_KEY --legacy src/ParachainStaking.sol:ParachainStaking | grep "^Deployed to: " | tail -c 43 || exit`
echo Staking contract deployed to "$STAKING"

# Deploy governance contract
echo Deploying governance contract...
GOVERNANCE=`forge create --rpc-url http://localhost:9921 --constructor-args "$REGISTRY" "$STAKING" "$MULTISIG" --private-key $PRIVATE_KEY --legacy src/ParachainGovernance.sol:ParachainGovernance | grep "^Deployed to: " | tail -c 43 || exit`
echo Governance contract deployed to "$GOVERNANCE"

cd ..

# Initialise remaining network state
echo Initialising network...
parachains-integration-tests -m test -t network-init.yaml --action-delay 0