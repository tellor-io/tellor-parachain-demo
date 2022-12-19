echo Building Polkadot...
cd polkadot
cargo build --release
cd ..

echo Building Moonbeam...
cd moonbeam
git checkout runtime-2000
cargo build --release
cd ..

echo Building Tellor contracts...
cd tellor-contracts
forge build
cd ..