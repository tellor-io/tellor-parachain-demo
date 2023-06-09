#!/bin/bash
set -euxo pipefail

echo Fetching Polkadot...
rm polkadot/target/release/polkadot
mkdir -p polkadot/target/release && \
wget -q "https://github.com/paritytech/polkadot/releases/download/v0.9.41/polkadot" -O polkadot/target/release/polkadot
chmod +x polkadot/target/release/polkadot

echo Fetching Statemine...
rm cumulus/target/release/polkadot-parachain
mkdir -p cumulus/target/release && \
wget -q "https://github.com/paritytech/cumulus/releases/download/v0.9.400/polkadot-parachain" -O cumulus/target/release/polkadot-parachain
chmod +x cumulus/target/release/polkadot-parachain

echo Fetching Moonbeam...
rm moonbeam/target/release/moonbeam
mkdir -p moonbeam/target/release && \
wget -q "https://github.com/PureStake/moonbeam/releases/download/v0.31.1/moonbeam" -O moonbeam/target/release/moonbeam
chmod +x moonbeam/target/release/moonbeam

echo Building consumer parachain...
cd substrate-parachain-node || exit
cargo build --release
cd ..