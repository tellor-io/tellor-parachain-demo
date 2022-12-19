#!/bin/bash

echo Building Polkadot...
cd polkadot || exit
cargo build --release
cd ..

echo Building Moonbeam...
cd moonbeam || exit
git checkout runtime-2000
cargo build --release
cd ..

echo Building consumer parachain...
cd substrate-parachain-node || exit
cargo build --release
cd ..

echo Building Tellor contracts...
cd tellor-contracts || exit
forge build
cd ..