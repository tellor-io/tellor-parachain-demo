use super::*;
use beefy_primitives::crypto::AuthorityId as BeefyId;
use grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_primitives::{AssignmentId, ValidatorId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_runtime::BuildStorage;

use rococo_runtime_constants::currency::UNITS as ROC;
const ENDOWMENT: u128 = 1_000_000 * ROC;

pub fn genesis() -> Storage {
    let genesis_config = rococo_runtime::GenesisConfig {
        system: rococo_runtime::SystemConfig {
            code: rococo_runtime::WASM_BINARY.unwrap().to_vec(),
            ..Default::default()
        },
        balances: rococo_runtime::BalancesConfig {
            balances: accounts::init_balances()
                .iter()
                .map(|k| (k.clone(), ENDOWMENT))
                .collect(),
        },
        session: rococo_runtime::SessionConfig {
            keys: validators::initial_authorities()
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                            get_from_seed::<BeefyId>("Alice"),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        babe: rococo_runtime::BabeConfig {
            authorities: Default::default(),
            epoch_config: Some(rococo_runtime::BABE_GENESIS_EPOCH_CONFIG),
            ..Default::default()
        },
        sudo: rococo_runtime::SudoConfig {
            key: Some(crate::Rococo::account_id_of(ALICE)),
        },
        configuration: rococo_runtime::ConfigurationConfig {
            config: get_host_config(),
        },
        registrar: rococo_runtime::RegistrarConfig {
            next_free_para_id: polkadot_primitives::LOWEST_PUBLIC_ID,
            ..Default::default()
        },
        ..Default::default()
    };

    genesis_config.build_storage().unwrap()
}

fn get_host_config() -> HostConfiguration<BlockNumber> {
    HostConfiguration {
        max_upward_queue_size: 51200,
        max_upward_message_size: 51200,
        max_upward_message_num_per_candidate: 10,
        max_downward_message_size: 51200,
        ..Default::default()
    }
}

fn session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    im_online: ImOnlineId,
    para_validator: ValidatorId,
    para_assignment: AssignmentId,
    authority_discovery: AuthorityDiscoveryId,
    beefy: BeefyId,
) -> rococo_runtime::SessionKeys {
    rococo_runtime::SessionKeys {
        babe,
        grandpa,
        im_online,
        para_validator,
        para_assignment,
        authority_discovery,
        beefy,
    }
}
