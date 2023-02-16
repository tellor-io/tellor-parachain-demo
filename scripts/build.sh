#!/bin/bash

echo Building Polkadot...
cd polkadot || exit
cargo build --release
cd ..

echo Building Statemine...
cd cumulus || exit
cargo build --release
cd ..

echo Building Moonbeam...
cd moonbeam || exit
cargo build --release
cd ..

echo Building consumer parachain...
cd substrate-parachain-node || exit
cargo build --release
cd ..