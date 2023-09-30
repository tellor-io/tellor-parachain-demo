use frame_support::{sp_io, sp_tracing};
use integration_tests_common::{constants::*, *};
use sp_core::{ecdsa, sr25519, storage::Storage, Pair, Public};
use sp_runtime::BuildStorage;
use tellor::Address;
use xcm::prelude::*;
use xcm_emulator::{
    decl_test_networks, decl_test_parachains, decl_test_relay_chains, AccountIdOf, ParaId,
    Parachain, RelayChain, TestExt,
};
use xcm_executor::traits::Convert;

mod parachains;
mod relay_chain;
#[cfg(test)]
mod tests;

decl_test_relay_chains! {
    pub struct Rococo {
        // Until updating to 1.0, when this can be replaced with integration_tests_common::constants::rococo::genesis()
        genesis = relay_chain::genesis(),
        on_init = (),
        runtime = {
            Runtime: rococo_runtime::Runtime,
            RuntimeOrigin: rococo_runtime::RuntimeOrigin,
            RuntimeCall: rococo_runtime::RuntimeCall,
            RuntimeEvent: rococo_runtime::RuntimeEvent,
            MessageQueue: rococo_runtime::MessageQueue,
            XcmConfig: rococo_runtime::xcm_config::XcmConfig,
            SovereignAccountOf: rococo_runtime::xcm_config::LocationConverter,
            System: rococo_runtime::System,
            Balances: rococo_runtime::Balances,
        },
        pallets_extra = {}
    }
}

decl_test_parachains! {
    pub struct AssetReserveParachain {
        genesis = constants::statemine::genesis(),
        on_init = (),
        runtime = {
            Runtime: statemine_runtime::Runtime,
            RuntimeOrigin: statemine_runtime::RuntimeOrigin,
            RuntimeCall: statemine_runtime::RuntimeCall,
            RuntimeEvent: statemine_runtime::RuntimeEvent,
            XcmpMessageHandler: statemine_runtime::XcmpQueue,
            DmpMessageHandler: statemine_runtime::DmpQueue,
            LocationToAccountId: statemine_runtime::xcm_config::LocationToAccountId,
            System: statemine_runtime::System,
            Balances: statemine_runtime::Balances,
            ParachainSystem: statemine_runtime::ParachainSystem,
            ParachainInfo: statemine_runtime::ParachainInfo,
        },
        pallets_extra = {}
    },
    pub struct EvmParachain {
        genesis = parachains::evm::genesis(),
        on_init = parachains::evm::init(),
        runtime = {
            Runtime: moonbeam_runtime::Runtime,
            RuntimeOrigin: moonbeam_runtime::RuntimeOrigin,
            RuntimeCall: moonbeam_runtime::RuntimeCall,
            RuntimeEvent: moonbeam_runtime::RuntimeEvent,
            XcmpMessageHandler: moonbeam_runtime::XcmpQueue,
            DmpMessageHandler: moonbeam_runtime::DmpQueue,
            LocationToAccountId: moonbeam_runtime::xcm_config::LocationToAccountId,
            System: moonbeam_runtime::System,
            Balances: moonbeam_runtime::Balances,
            ParachainSystem: moonbeam_runtime::ParachainSystem,
            ParachainInfo: moonbeam_runtime::ParachainInfo,
        },
        pallets_extra = {}
    },
    pub struct OracleConsumerParachain {
        genesis = parachains::oracle_consumer::genesis(),
        on_init = parachains::oracle_consumer::init(),
        runtime = {
            Runtime: oracle_consumer_runtime::Runtime,
            RuntimeOrigin: oracle_consumer_runtime::RuntimeOrigin,
            RuntimeCall: oracle_consumer_runtime::RuntimeCall,
            RuntimeEvent: oracle_consumer_runtime::RuntimeEvent,
            XcmpMessageHandler: oracle_consumer_runtime::XcmpQueue,
            DmpMessageHandler: oracle_consumer_runtime::DmpQueue,
            LocationToAccountId: oracle_consumer_runtime::xcm_config::LocationToAccountId,
            System: oracle_consumer_runtime::System,
            Balances: oracle_consumer_runtime::Balances,
            ParachainSystem: oracle_consumer_runtime::ParachainSystem,
            ParachainInfo: oracle_consumer_runtime::ParachainInfo,
        },
        pallets_extra = {
            Tellor: oracle_consumer_runtime::Tellor,
        }
    }
}

decl_test_networks! {
    pub struct RococoMockNet {
        relay_chain = Rococo,
        parachains = vec![
            AssetReserveParachain,
            EvmParachain,
            OracleConsumerParachain,
        ],
    }
}

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_20_from_seed(seed: &str) -> account::AccountId20 {
    use sp_runtime::traits::IdentifyAccount;
    let pubkey = ecdsa::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public();
    account::EthereumSigner::from(pubkey).into_account()
}
