// subxt codegen --url http://localhost:9910 | rustfmt > ./src/chains/asset_hub.rs
pub(crate) mod asset_hub;
// subxt codegen --url http://localhost:9920 | rustfmt > ./src/chains/evm.rs
pub(crate) mod evm;
// subxt codegen --url http://localhost:9930 | rustfmt > ./src/chains/oracle_consumer.rs
pub(crate) mod oracle_consumer;
