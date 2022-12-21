#!/bin/bash

echo Building Tellor contracts...
cd tellor-contracts || exit
forge build
echo Deploying Tellor contracts to Moonbeam...
forge create --rpc-url http://localhost:9931 --constructor-args 0xFFFFFFFFxcTok3nAddr355 \
  --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --legacy src/Staking.sol:Staking
cd ..