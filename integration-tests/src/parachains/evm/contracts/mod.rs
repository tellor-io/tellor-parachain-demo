use super::*;
use ethabi::{encode, ethereum_types::H256, Event, EventParam, Function, Param, ParamType, Token};
use frame_support::assert_ok;
use hex_literal::hex;
use lazy_static::lazy_static;
use pallet_evm::{ExitReason::Succeed, ExitSucceed::Stopped};
use sp_runtime::app_crypto::sp_core::U256;
use with_builtin_macros::with_builtin;

pub(crate) mod governance;
pub(crate) mod registry;
pub(crate) mod staking;

const GAS_LIMIT: u64 = 10_000_000;
const MAX_FEE_PER_GAS: u128 = 125_000_000_000;
