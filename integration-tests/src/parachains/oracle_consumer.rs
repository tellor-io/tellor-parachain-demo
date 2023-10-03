use super::*;
use core::time::Duration;
use ethabi::ethereum_types::H256;
use frame_support::{
    assert_ok,
    traits::{fungible::Inspect, UnixTime},
    BoundedVec,
};
use integration_tests_common::constants::accounts;
use oracle_consumer_runtime::{
    Balance, Balances, BalancesConfig, CollatorSelectionConfig, GenesisConfig, ParachainInfoConfig,
    PolkadotXcmConfig, Runtime, RuntimeOrigin, SessionConfig, SessionKeys, System, SystemConfig,
    Tellor, Timestamp, WASM_BINARY,
};
use sp_runtime::{
    traits::{AccountIdConversion, Hash, Keccak256},
    AccountId32,
};

lazy_static! {
    pub(crate) static ref BOB: AccountId = OracleConsumerParachain::account_id_of(accounts::BOB);
    pub(crate) static ref CHARLIE: AccountId =
        OracleConsumerParachain::account_id_of(accounts::CHARLIE);
    pub(crate) static ref DAVE: AccountId = OracleConsumerParachain::account_id_of(accounts::DAVE);
}

pub(crate) fn genesis() -> Storage {
    const PARA_ID: ParaId = ParaId::new(3_000);

    let pallet_id =
        <<OracleConsumerParachain as Parachain>::Runtime as tellor::Config>::PalletId::get();

    let genesis_config = GenesisConfig {
        system: SystemConfig {
            code: WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: vec![
                (
                    // required to claim tips
                    BOB.clone(),
                    Balances::minimum_balance(),
                ),
                (
                    // required for tips
                    CHARLIE.clone(),
                    10 * 10u128.pow(12),
                ),
                (
                    // required for disputes
                    DAVE.clone(),
                    55 * 10u128.pow(12),
                ),
                // required for fees
                (pallet_id.into_account_truncating(), 1 * 10u128.pow(12)),
                // initialise sub-accounts
                (
                    pallet_id.into_sub_account_truncating(b"tips"),
                    Balances::minimum_balance(),
                ),
                (
                    pallet_id.into_sub_account_truncating(b"staking"),
                    Balances::minimum_balance(),
                ),
            ],
        },
        collator_selection: CollatorSelectionConfig {
            invulnerables: collators::invulnerables()
                .iter()
                .cloned()
                .map(|(acc, _)| acc)
                .collect(),
            candidacy_bond: Balances::minimum_balance() * 16,
            ..Default::default()
        },
        parachain_info: ParachainInfoConfig {
            parachain_id: PARA_ID,
            ..Default::default()
        },
        session: SessionConfig {
            keys: collators::invulnerables()
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),          // account id
                        acc,                  // validator id
                        SessionKeys { aura }, // session keys
                    )
                })
                .collect(),
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
    pallet_timestamp::Now::<<OracleConsumerParachain as xcm_emulator::Parachain>::Runtime>::put(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Current time is always after unix epoch; qed")
            .as_millis() as u64,
    );
}

pub(crate) fn feed_id(
    query_id: H256,
    reward: Balance,
    start_time: u64,
    interval: u64,
    window: u64,
    price_threshold: u16,
    reward_increase_per_second: Balance,
) -> H256 {
    use ethabi::Token::*;
    Keccak256::hash(&ethabi::encode(&vec![
        FixedBytes(query_id.0.to_vec()),
        Uint(reward.into()),
        Uint(start_time.into()),
        Uint(interval.into()),
        Uint(window.into()),
        Uint(price_threshold.into()),
        Uint(reward_increase_per_second.into()),
    ]))
}

pub(crate) fn register(evm_para_id: impl Into<u32>) {
    use tellor::{weights::WeightInfo, MAX_VOTE_ROUNDS};
    assert_ok!(Tellor::register(RuntimeOrigin::root(), None));
    let weights = tellor::Weights {
        report_stake_deposited: <() as WeightInfo>::report_stake_deposited().ref_time(),
        report_staking_withdraw_request: <() as WeightInfo>::report_staking_withdraw_request()
            .ref_time(),
        report_stake_withdrawn: <() as WeightInfo>::report_stake_withdrawn().ref_time(),
        report_vote_tallied: <() as WeightInfo>::report_vote_tallied().ref_time(),
        report_vote_executed: <() as WeightInfo>::report_vote_executed(MAX_VOTE_ROUNDS.into())
            .ref_time(),
        report_slash: <() as WeightInfo>::report_slash().ref_time(),
    };
    System::assert_has_event(
        tellor::Event::RegistrationSent {
            para_id: evm_para_id.into(),
            contract_address: *evm::contracts::registry::REGISTRY_CONTRACT_ADDRESS,
            weights,
        }
        .into(),
    );
}

pub(crate) fn submit_value(
    reporter: AccountId32,
    query_data: &[u8],
    value: &[u8],
) -> (tellor::QueryId, tellor::Timestamp) {
    type QueryData = BoundedVec<u8, <Runtime as tellor::Config>::MaxQueryDataLength>;
    type Value = BoundedVec<u8, <Runtime as tellor::Config>::MaxValueLength>;

    let query_data: QueryData = query_data.to_vec().try_into().unwrap();
    let query_id = Keccak256::hash(query_data.as_slice());
    let value: Value = value.to_vec().try_into().unwrap();
    let nonce = 0;
    let timestamp = <Timestamp as UnixTime>::now().as_secs();

    assert_ok!(Tellor::submit_value(
        RuntimeOrigin::signed(reporter),
        query_id,
        value.clone(),
        nonce,
        query_data.clone()
    ));
    System::assert_has_event(
        tellor::Event::NewReport {
            query_id,
            time: timestamp,
            value,
            nonce,
            query_data,
            reporter: BOB.clone(),
        }
        .into(),
    );
    (query_id, timestamp)
}

pub(crate) fn advance_time(time_in_secs: u64) {
    let now = <Timestamp as UnixTime>::now();
    pallet_timestamp::Now::<Runtime>::set(
        (now + Duration::from_secs(time_in_secs)).as_millis() as u64
    );
}
