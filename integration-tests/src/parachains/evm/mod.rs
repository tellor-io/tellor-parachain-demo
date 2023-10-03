use super::*;
use account::AccountId20;
use core::time::Duration;
use frame_support::{assert_ok, traits::UnixTime};
use moonbeam_runtime::{
    asset_config::AssetRegistrarMetadata, xcm_config::AssetType, AssetManager, BalancesConfig,
    EVMConfig, GenesisAccount, GenesisConfig, ParachainInfoConfig, PolkadotXcmConfig, Precompiles,
    Runtime, RuntimeEvent, RuntimeOrigin, System, SystemConfig, Timestamp, EVM, WASM_BINARY,
};
use sp_runtime::{app_crypto::sp_core::bytes::from_hex, app_crypto::sp_core::H160};
use xcm::prelude::{GeneralIndex, PalletInstance, Parachain};
use xcm::v3::{Junctions, MultiLocation};

pub(crate) mod contracts;

lazy_static! {
    // https://github.com/moonbeam-foundation/moonbeam#prefunded-development-addresses
    pub(crate) static ref ALITH: Address =
        address_of("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac");
    pub(crate) static ref BALTHAZAR: Address =
        address_of("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0");
    pub(crate) static ref CHARLETH: Address =
        address_of("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc");
    pub(crate) static ref DOROTHY: Address =
        address_of("773539d4Ac0e786233D90A233654ccEE26a613D9");
    pub(crate) static ref PALLET_DERIVATIVE_ACCOUNT: Address =
        address_of("26ab7997cf6d531fed12b2fa6bc3cf2248724195");
    pub(crate) static ref XCTRB_ADDRESS: Address =
        address_of("ffffffffc8be577a279484431b9444687ec3d2ae");
}

fn address_of(address: &str) -> H160 {
    use std::str::FromStr;
    H160::from_str(address).expect("internal H160 is valid; qed")
}

pub(crate) fn genesis() -> Storage {
    const PARA_ID: ParaId = ParaId::new(2_000);

    // set precompiles revert bytecode: https://github.com/PureStake/moonbeam/blob/a814fcf36a67f0f14f40afcd7d12fd4f3c5e775b/node/service/src/chain_spec/moonbeam.rs#L244
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

    let genesis_config = GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: vec![
                ((*ALITH).into(), 2 * 10u128.saturating_pow(18)), // contract deployment
                ((*BALTHAZAR).into(), 2 * 10u128.saturating_pow(18)), // contract transactions
                (
                    (*PALLET_DERIVATIVE_ACCOUNT).into(),
                    1 * 10u128.saturating_pow(18), // required for xcm fees
                ),
            ],
        },
        evm: EVMConfig {
            // We need _some_ code inserted at the precompile address so evm will actually call the address
            accounts: Precompiles::used_addresses()
                .map(|addr| {
                    (
                        addr.into(),
                        GenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: revert_bytecode.clone(),
                        },
                    )
                })
                .collect(),
        },
        parachain_info: ParachainInfoConfig {
            parachain_id: PARA_ID,
            ..Default::default()
        },
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(XCM_VERSION),
            ..Default::default()
        },
        ..Default::default()
    };
    genesis_config.build_storage().unwrap()
}

pub(crate) fn init() {
    pallet_timestamp::Now::<<EvmParachain as xcm_emulator::Parachain>::Runtime>::put(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Current time is always after unix epoch; qed")
            .as_millis() as u64,
    );
}

pub(crate) fn advance_time(time_in_secs: u64) {
    let now = <Timestamp as UnixTime>::now();
    pallet_timestamp::Now::<Runtime>::set(
        (now + Duration::from_secs(time_in_secs)).as_millis() as u64
    );
}

pub(crate) fn create_xctrb_asset() {
    let asset = AssetType::Xcm(MultiLocation {
        parents: 1,
        interior: Junctions::X3(Parachain(1_000), PalletInstance(50), GeneralIndex(872)),
    });
    let metadata = AssetRegistrarMetadata {
        name: b"Tellor Tribute".to_vec(),
        symbol: b"xcTRB".to_vec(),
        decimals: 18,
        is_frozen: false,
    };
    // register asset
    assert_ok!(AssetManager::register_foreign_asset(
        RuntimeOrigin::root(),
        asset.clone(),
        metadata.clone(),
        1,
        true
    ));
    let asset_id = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
    System::assert_last_event(
        pallet_asset_manager::Event::ForeignAssetRegistered {
            asset_id,
            asset: asset.clone(),
            metadata,
        }
        .into(),
    );
    // set units per second
    let units_per_second = 100_000; // todo: size correctly
    assert_ok!(AssetManager::set_asset_units_per_second(
        RuntimeOrigin::root(),
        asset.clone(),
        units_per_second,
        5, // todo: size correctly
    ));
    System::assert_last_event(
        pallet_asset_manager::Event::UnitsPerSecondChanged {
            asset_type: asset,
            units_per_second,
        }
        .into(),
    );
    // push revert code to EVM: https://github.com/PureStake/xcm-tools/blob/4a3dcdb49434bcc019106677d01be54f9f17b30b/scripts/xcm-asset-registrator.ts#L87-L107
    // required for transferFrom usage within a smart contract, especially to return revert specifics
    assert_ok!(System::set_storage(
        RuntimeOrigin::root(),
        vec![(
            from_hex("0x1da53b775b270400e7e61ed5cbc5a146ea70f53d5a3306ce02aaf97049cf181a8d25f78201571c23f3f5096d309394ecffffffffc8be577a279484431b9444687ec3d2ae").unwrap(),
            from_hex("0x1460006000fd").unwrap()
        )]
    ));
}
