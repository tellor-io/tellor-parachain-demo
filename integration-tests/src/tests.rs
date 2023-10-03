use super::*;
use frame_support::{assert_ok, traits::Get, traits::Hooks};
use oracle_consumer_runtime::Tellor;
use parachains::evm::{
    contracts::governance::GOVERNANCE_CONTRACT_ADDRESS,
    contracts::registry::REGISTRY_CONTRACT_ADDRESS, contracts::staking::STAKING_CONTRACT_ADDRESS,
    BALTHAZAR, DOROTHY, PALLET_DERIVATIVE_ACCOUNT, XCTRB_ADDRESS,
};
use parachains::oracle_consumer::{BOB, CHARLIE, DAVE};
use sp_runtime::{
    app_crypto::ByteArray,
    traits::{Hash, Keccak256},
};
use std::sync::Once;
use tellor::{VoteResult, DAYS, HOURS, MINUTES};
use xcm_emulator::TestExt;

static INIT: Once = Once::new();
fn init_tracing() {
    INIT.call_once(|| {
        // Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_env_filter("xcm=trace,system::events=trace,evm=trace") // Comment out this line to see all traces
            .with_test_writer()
            .init();
    });
}

#[test]
fn creating_xctrb_on_evm_parachain_works() {
    init_tracing();

    // create TRB as foreign asset
    EvmParachain::execute_with(|| parachains::evm::create_xctrb_asset());
}

#[test]
fn deploying_contracts_to_evm_parachain_works() {
    init_tracing();

    // deploy and init contracts
    EvmParachain::execute_with(|| {
        use parachains::{evm::contracts::*, evm::ALITH};
        // deploy contracts
        registry::deploy();
        staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
        governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
        // init contracts with addresses
        staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        governance::init(*STAKING_CONTRACT_ADDRESS);
    });
}

#[test]
fn register_on_consumer_parachain_registers_with_contracts_on_evm_parachain() {
    init_tracing();

    // deploy parachain registry contract to evm parachain
    EvmParachain::execute_with(|| {
        parachains::evm::contracts::registry::deploy();
    });

    // register oracle consumer parachain with contracts on evm parachain via tellor pallet
    OracleConsumerParachain::execute_with(|| {
        parachains::oracle_consumer::register(EvmParachain::para_id())
    });

    // ensure registry contract called on evm parachain and expected events emitted
    EvmParachain::execute_with(|| {
        use parachains::evm::contracts::registry;
        // ensure registry contract called (via pallet derivative account on evm parachain)
        registry::assert_executed(*PALLET_DERIVATIVE_ACCOUNT);
        // ensure ParachainRegistered event emitted by parachain registry contract
        registry::assert_parachain_registered_event(OracleConsumerParachain::para_id());
    });
}

mod staking {
    use super::*;

    #[test]
    fn deposit_stake_on_evm_parachain_reports_to_consumer_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        let amount =
            <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            // deposit stake
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
            staking::assert_new_staker_event(*BALTHAZAR, amount);
            staking::assert_new_parachain_staker_event(
                OracleConsumerParachain::para_id(),
                *BALTHAZAR,
                BOB.to_raw_vec(),
                amount,
            );
        });

        // ensure stake reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::NewStakerReported {
                    staker: BOB.clone(),
                    amount: amount.into(),
                    address: *BALTHAZAR,
                }
                .into(),
            );
        });
    }

    #[test]
    fn submit_value_to_consumer_parachain_after_staking_works() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        let amount =
            <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // submit value to oracle
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::NewStakerReported {
                    staker: BOB.clone(),
                    amount: amount.into(),
                    address: *BALTHAZAR,
                }
                .into(),
            );
            parachains::oracle_consumer::submit_value(BOB.clone(), b"hello tellor", b"hey!")
        });
    }

    #[test]
    fn requesting_stake_withdraw_on_evm_parachain_reports_to_consumer_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve, stake trb and request withdrawal from staking contract for oracle consumer parachain
        let amount =
            <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
            // request withdraw
            staking::request_parachain_stake_withdraw(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                amount,
            );
            staking::assert_stake_withdraw_requested_event(*BALTHAZAR, amount);
            staking::assert_parachain_stake_withdraw_requested_event(
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // ensure stake withdraw request reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::StakeWithdrawRequestReported {
                    reporter: BOB.clone(),
                    amount: amount.into(),
                    address: *BALTHAZAR,
                }
                .into(),
            );
        });
    }

    #[test]
    fn withdraw_stake_on_evm_parachain_reports_to_consumer_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve, stake trb and request withdrawal from staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
            // request withdraw
            staking::request_parachain_stake_withdraw(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                amount,
            );
            staking::assert_stake_withdraw_requested_event(*BALTHAZAR, amount);
            staking::assert_parachain_stake_withdraw_requested_event(
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // advance time beyond lock period on both parachains
        EvmParachain::execute_with(|| parachains::evm::advance_time((7 * DAYS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((7 * DAYS) + 1)
        });

        // withdraw stake from staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            staking::withdraw_parachain_stake(*BALTHAZAR, OracleConsumerParachain::para_id());
            staking::assert_stake_withdrawn_event(*BALTHAZAR);
            staking::assert_parachain_stake_withdrawn_event(
                OracleConsumerParachain::para_id(),
                *BALTHAZAR,
            );
        });

        // ensure stake withdrawal reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::StakeWithdrawnReported {
                    staker: BOB.clone(),
                }
                .into(),
            );
        });
    }
}

mod autopay {
    use super::*;
    use codec::Compact;
    use frame_support::bounded_vec;

    #[test]
    fn claim_onetime_tip_on_consumer_parachain_works() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // add tip, submit value to oracle and then claim
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, System, Tellor};
            let query_data = b"hello tellor";
            let query_id = Keccak256::hash(query_data);
            let amount = 1_000;
            // create onetime tip
            assert_ok!(Tellor::tip(
                RuntimeOrigin::signed(CHARLIE.clone()),
                query_id,
                amount,
                query_data.to_vec().try_into().unwrap()
            ));
            System::assert_has_event(
                tellor::Event::TipAdded {
                    query_id,
                    amount,
                    query_data: query_data.to_vec().try_into().unwrap(),
                    tipper: CHARLIE.clone(),
                }
                .into(),
            );
            // submit value (next block)
            parachains::oracle_consumer::advance_time(1);
            parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // advance time until claim buffer passed
            parachains::oracle_consumer::advance_time((12 * HOURS) + 1);
            // claim tip
            assert_ok!(Tellor::claim_onetime_tip(
                RuntimeOrigin::signed(BOB.clone()),
                query_id,
                bounded_vec![Compact(Tellor::time_of_last_new_value().unwrap())]
            ));
            System::assert_has_event(
                tellor::Event::OneTimeTipClaimed {
                    query_id,
                    amount,
                    reporter: BOB.clone(),
                }
                .into(),
            );
        });
    }

    #[test]
    fn claim_tip_on_consumer_parachain_works() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // create feed, submit value to oracle and then claim
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{AccountId, RuntimeOrigin, System, Tellor};
            let query_data = b"hello tellor";
            let query_id = Keccak256::hash(query_data);
            let amount = 10_000;
            // setup data feed
            let reward = 1_000;
            let start_time = <Tellor as tellor::UsingTellor<AccountId>>::now();
            let interval = 600;
            let window = 12;
            let price_threshold = 0;
            let reward_increase_per_second = 0;
            assert_ok!(Tellor::setup_data_feed(
                RuntimeOrigin::signed(CHARLIE.clone()),
                query_id,
                reward,
                start_time,
                interval,
                window,
                price_threshold,
                reward_increase_per_second,
                query_data.to_vec().try_into().unwrap(),
                amount
            ));
            let feed_id = parachains::oracle_consumer::feed_id(
                query_id,
                reward,
                start_time,
                interval,
                window,
                price_threshold,
                reward_increase_per_second,
            );
            System::assert_has_event(
                tellor::Event::NewDataFeed {
                    query_id,
                    feed_id,
                    query_data: query_data.to_vec().try_into().unwrap(),
                    feed_creator: CHARLIE.clone(),
                }
                .into(),
            );
            // submit value
            parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // advance time until claim buffer passed
            parachains::oracle_consumer::advance_time((12 * HOURS) + 1);
            // claim tip
            assert_ok!(Tellor::claim_tip(
                RuntimeOrigin::signed(BOB.clone()),
                feed_id,
                query_id,
                bounded_vec![Compact(Tellor::time_of_last_new_value().unwrap())]
            ));
            System::assert_has_event(
                tellor::Event::TipClaimed {
                    feed_id,
                    query_id,
                    amount: reward,
                    reporter: BOB.clone(),
                }
                .into(),
            );
        });
    }
}

mod governance {
    use super::*;

    #[test]
    fn begin_dispute_on_consumer_parachain_begins_dispute_on_evm_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
            governance::init(*STAKING_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        let amount =
            <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // submit value to oracle consumer parachain and then begin dispute of reported value
        let (query_id, timestamp) = OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{Runtime, RuntimeOrigin, System, Tellor};
            // submit value
            let (query_id, timestamp) =
                parachains::oracle_consumer::submit_value(BOB.clone(), b"hello tellor", b"hey!");
            // begin dispute
            assert_ok!(Tellor::begin_dispute(
                RuntimeOrigin::signed(DAVE.clone()),
                query_id,
                timestamp,
                Some(*DOROTHY)
            ));
            let dispute_id = Keccak256::hash(&ethabi::encode(&[
                ethabi::Token::Uint(<Runtime as tellor::Config>::ParachainId::get().into()),
                ethabi::Token::FixedBytes(query_id.0.into()),
                ethabi::Token::Uint(timestamp.into()),
            ]));
            System::assert_has_event(
                tellor::Event::NewDispute {
                    dispute_id,
                    query_id,
                    timestamp,
                    reporter: BOB.clone(),
                }
                .into(),
            );
            System::assert_has_event(
                tellor::Event::NewDisputeSent {
                    para_id: EvmParachain::para_id().into(),
                    contract_address: *GOVERNANCE_CONTRACT_ADDRESS,
                }
                .into(),
            );
            (query_id, timestamp)
        });

        // ensure governance contract called and events emitted on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            // ensure governance contract called (via pallet derivative account on evm parachain)
            contracts::governance::assert_executed(*PALLET_DERIVATIVE_ACCOUNT);
            // ensure ParachainReporterSlashed event emitted by parachain staking contract
            contracts::staking::assert_parachain_reporter_slashed_event(
                OracleConsumerParachain::para_id(),
                *BALTHAZAR,
                *GOVERNANCE_CONTRACT_ADDRESS,
                amount,
            );
            // ensure NewParachainDispute event emitted by parachain governance contract
            contracts::governance::assert_new_parachain_dispute_event(
                OracleConsumerParachain::para_id(),
                query_id.0.to_vec(),
                timestamp,
                *BALTHAZAR,
            );
        });

        // ensure slash reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::SlashReported {
                    reporter: BOB.clone(),
                    amount: amount.into(),
                }
                .into(),
            );
        });
    }

    #[test]
    fn on_initialize_hook_on_consumer_parachain_sends_votes_to_evm_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
            governance::init(*STAKING_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // create tip to be able to have vote counted as user
        let tip = 1_000_000_000_000;
        let query_data = b"hello tellor";
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, Tellor};
            assert_ok!(Tellor::tip(
                RuntimeOrigin::signed(DAVE.clone()),
                Keccak256::hash(query_data.as_slice()),
                tip,
                query_data.to_vec().try_into().unwrap()
            ));
        });

        // submit value to oracle consumer parachain, begin dispute and cast votes
        let dispute_id = OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{Runtime, RuntimeOrigin, Tellor};
            // submit value
            let (query_id, timestamp) =
                parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // begin dispute
            assert_ok!(Tellor::begin_dispute(
                RuntimeOrigin::signed(DAVE.clone()),
                query_id,
                timestamp,
                Some(*DOROTHY)
            ));
            let dispute_id = Keccak256::hash(&ethabi::encode(&[
                ethabi::Token::Uint(<Runtime as tellor::Config>::ParachainId::get().into()),
                ethabi::Token::FixedBytes(query_id.0.into()),
                ethabi::Token::Uint(timestamp.into()),
            ]));
            // cast votes
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(DAVE.clone()),
                dispute_id,
                Some(true) // for
            ));
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(BOB.clone()),
                dispute_id,
                Some(false) // against
            ));
            dispute_id
        });

        // advance time until parachain voting cut-off
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((11 * HOURS) + 1);
            Tellor::on_initialize(0)
        });

        // ensure governance contract called and events emitted on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            // ensure governance contract called (via pallet derivative account on evm parachain)
            contracts::governance::assert_executed(*PALLET_DERIVATIVE_ACCOUNT);
            // ensure ParachainVoted event emitted by parachain governance contract
            contracts::governance::assert_parachain_voted_event(dispute_id, tip, 0, 0, 0, 1, 0);
        });
    }

    #[test]
    fn send_votes_from_consumer_parachain_to_evm_parachain_works() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
            governance::init(*STAKING_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // create tip to be able to have vote counted as user
        let tip = 1_000_000_000_000;
        let query_data = b"hello tellor";
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, Tellor};
            assert_ok!(Tellor::tip(
                RuntimeOrigin::signed(DAVE.clone()),
                Keccak256::hash(query_data.as_slice()),
                tip,
                query_data.to_vec().try_into().unwrap()
            ));
        });

        // submit value to oracle consumer parachain, begin dispute and cast votes
        let dispute_id = OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{Runtime, RuntimeOrigin, Tellor};
            // submit value
            let (query_id, timestamp) =
                parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // begin dispute
            assert_ok!(Tellor::begin_dispute(
                RuntimeOrigin::signed(DAVE.clone()),
                query_id,
                timestamp,
                Some(*DOROTHY)
            ));
            let dispute_id = Keccak256::hash(&ethabi::encode(&[
                ethabi::Token::Uint(<Runtime as tellor::Config>::ParachainId::get().into()),
                ethabi::Token::FixedBytes(query_id.0.into()),
                ethabi::Token::Uint(timestamp.into()),
            ]));
            // cast votes
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(DAVE.clone()),
                dispute_id,
                Some(true) // for
            ));
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(BOB.clone()),
                dispute_id,
                Some(false) // against
            ));
            // advance time until parachain voting cut-off
            parachains::oracle_consumer::advance_time((11 * HOURS) + 1);
            assert_ok!(Tellor::send_votes(RuntimeOrigin::signed(DAVE.clone()), 5));
            dispute_id
        });

        // ensure governance contract called and events emitted on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            // ensure governance contract called (via pallet derivative account on evm parachain)
            contracts::governance::assert_executed(*PALLET_DERIVATIVE_ACCOUNT);
            // ensure ParachainVoted event emitted by parachain governance contract
            contracts::governance::assert_parachain_voted_event(dispute_id, tip, 0, 0, 0, 1, 0);
        });
    }

    #[test]
    fn tally_votes_on_evm_parachain_reports_result_to_consumer_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
            governance::init(*STAKING_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // create tip to be able to have vote counted as user
        let tip = 1_000_000_000_000;
        let query_data = b"hello tellor";
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, Tellor};
            assert_ok!(Tellor::tip(
                RuntimeOrigin::signed(DAVE.clone()),
                Keccak256::hash(query_data.as_slice()),
                tip,
                query_data.to_vec().try_into().unwrap()
            ));
        });

        // submit value to oracle consumer parachain, begin dispute and cast votes
        let dispute_id = OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{Runtime, RuntimeOrigin, Tellor};
            // submit value
            let (query_id, timestamp) =
                parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // begin dispute
            assert_ok!(Tellor::begin_dispute(
                RuntimeOrigin::signed(DAVE.clone()),
                query_id,
                timestamp,
                Some(*DOROTHY)
            ));
            let dispute_id = Keccak256::hash(&ethabi::encode(&[
                ethabi::Token::Uint(<Runtime as tellor::Config>::ParachainId::get().into()),
                ethabi::Token::FixedBytes(query_id.0.into()),
                ethabi::Token::Uint(timestamp.into()),
            ]));
            // cast votes
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(DAVE.clone()),
                dispute_id,
                Some(true) // for
            ));
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(BOB.clone()),
                dispute_id,
                Some(false) // against
            ));
            dispute_id
        });

        // advance time until parachain voting cut-off
        EvmParachain::execute_with(|| parachains::evm::advance_time((11 * HOURS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((11 * HOURS) + 1);
            // send votes via on_initialize hook
            Tellor::on_initialize(0)
        });

        // advance time until voting cut-off
        EvmParachain::execute_with(|| parachains::evm::advance_time((1 * DAYS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((1 * DAYS) + 1);
            // send votes via on_initialize hook
            Tellor::on_initialize(0)
        });

        // tally votes on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            contracts::governance::tally_votes(*ALITH, dispute_id);
            contracts::governance::assert_vote_tallied_event(
                dispute_id,
                VoteResult::Invalid as u8,
                *DOROTHY,
                *BALTHAZAR,
            );
        });

        // ensure tally reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::VoteTallied {
                    dispute_id,
                    result: VoteResult::Invalid,
                    initiator: DAVE.clone(),
                    reporter: BOB.clone(),
                }
                .into(),
            );
        });
    }

    #[test]
    fn execute_vote_on_evm_parachain_reports_to_consumer_parachain() {
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
            governance::init(*STAKING_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // create tip to be able to have vote counted as user
        let tip = 1_000_000_000_000;
        let query_data = b"hello tellor";
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, Tellor};
            assert_ok!(Tellor::tip(
                RuntimeOrigin::signed(DAVE.clone()),
                Keccak256::hash(query_data.as_slice()),
                tip,
                query_data.to_vec().try_into().unwrap()
            ));
        });

        // submit value to oracle consumer parachain, begin dispute and cast votes
        let dispute_id = OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{Runtime, RuntimeOrigin, Tellor};
            // submit value
            let (query_id, timestamp) =
                parachains::oracle_consumer::submit_value(BOB.clone(), query_data, b"hey!");
            // begin dispute
            assert_ok!(Tellor::begin_dispute(
                RuntimeOrigin::signed(DAVE.clone()),
                query_id,
                timestamp,
                Some(*DOROTHY)
            ));
            let dispute_id = Keccak256::hash(&ethabi::encode(&[
                ethabi::Token::Uint(<Runtime as tellor::Config>::ParachainId::get().into()),
                ethabi::Token::FixedBytes(query_id.0.into()),
                ethabi::Token::Uint(timestamp.into()),
            ]));
            // cast votes
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(DAVE.clone()),
                dispute_id,
                Some(true) // for
            ));
            assert_ok!(Tellor::vote(
                RuntimeOrigin::signed(BOB.clone()),
                dispute_id,
                Some(false) // against
            ));
            dispute_id
        });

        // advance time until parachain voting cut-off
        EvmParachain::execute_with(|| parachains::evm::advance_time((11 * HOURS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((11 * HOURS) + 1);
            // send votes via on_initialize hook
            Tellor::on_initialize(0)
        });

        // advance time until voting cut-off
        EvmParachain::execute_with(|| parachains::evm::advance_time((1 * DAYS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((1 * DAYS) + 1);
            // send votes via on_initialize hook
            Tellor::on_initialize(0)
        });

        // tally votes on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            contracts::governance::tally_votes(*ALITH, dispute_id);
            contracts::governance::assert_vote_tallied_event(
                dispute_id,
                VoteResult::Invalid as u8,
                *DOROTHY,
                *BALTHAZAR,
            );
        });

        // advance time until voting cut-off
        EvmParachain::execute_with(|| parachains::evm::advance_time((3 * DAYS) + 1));
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::advance_time((3 * DAYS) + 1);
        });

        // execute vote on evm parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::*;
            contracts::governance::execute_vote(*ALITH, dispute_id);
            contracts::governance::assert_vote_executed_event(
                dispute_id,
                VoteResult::Invalid as u8,
            );
        });

        // ensure executed vote reported to tellor pallet on oracle consumer parachain
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::System;
            System::assert_has_event(
                tellor::Event::VoteExecuted {
                    dispute_id,
                    result: VoteResult::Invalid,
                }
                .into(),
            );
        });
    }
}

mod using_tellor {
    use super::*;

    #[test]
    fn using_tellor_sample_works() {
        use tellor::U256;
        init_tracing();

        // create trb asset and deploy contracts
        EvmParachain::execute_with(|| {
            use parachains::{evm::contracts::*, evm::ALITH};
            // create asset
            parachains::evm::create_xctrb_asset();
            // deploy contracts
            registry::deploy();
            staking::deploy(*REGISTRY_CONTRACT_ADDRESS, *XCTRB_ADDRESS);
            governance::deploy(*REGISTRY_CONTRACT_ADDRESS, *ALITH);
            // init contracts with addresses
            staking::init(*GOVERNANCE_CONTRACT_ADDRESS);
        });

        // register oracle consumer parachain with contracts on evm parachain via tellor pallet
        OracleConsumerParachain::execute_with(|| {
            parachains::oracle_consumer::register(EvmParachain::para_id())
        });

        // mint, approve and stake trb in staking contract for oracle consumer parachain
        EvmParachain::execute_with(|| {
            use parachains::evm::contracts::staking;
            let asset = u128::from_be_bytes(XCTRB_ADDRESS[4..].try_into().unwrap());
            let amount =
                <oracle_consumer_runtime::Runtime as tellor::Config>::MinimumStakeAmount::get();
            staking::mint(asset, *BALTHAZAR, amount);
            staking::approve(*BALTHAZAR, asset, *STAKING_CONTRACT_ADDRESS, amount);
            staking::deposit_parachain_stake(
                *BALTHAZAR,
                OracleConsumerParachain::para_id(),
                BOB.to_raw_vec(),
                amount,
            );
        });

        // submit price to oracle which is then used via using-tellor sample pallet to do something
        OracleConsumerParachain::execute_with(|| {
            use oracle_consumer_runtime::{RuntimeOrigin, System, UsingTellor};

            // configure using-tellor pallet with price source
            let query_data = b"DOT/USD";
            let query_id = Keccak256::hash(query_data.as_slice());
            assert_ok!(UsingTellor::configure(RuntimeOrigin::root(), query_id));
            System::assert_has_event(::using_tellor::Event::Configured { query_id }.into());

            // submit price to oracle
            let price = U256::from((4.39 * 10u64.pow(18) as f64) as u128);
            parachains::oracle_consumer::submit_value(
                BOB.clone(),
                query_data,
                &ethabi::encode(&vec![ethabi::Token::Uint(price)]),
            );

            // advance time, as using-tellor sample uses a delayed price to allow time for disputes
            parachains::oracle_consumer::advance_time((15 * MINUTES) + 1);

            // do something using previously submitted oracle price
            let value = U256::from(10);
            assert_ok!(UsingTellor::do_something(
                RuntimeOrigin::signed(CHARLIE.clone()),
                value
            ));
            System::assert_last_event(
                ::using_tellor::Event::ValueStored {
                    value: price * value,
                    who: CHARLIE.clone(),
                }
                .into(),
            );
        });
    }
}
