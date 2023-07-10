//RGB-E20
use amplify::confinement::LargeVec;
use bp::bc::stl::bitcoin_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::{
    AssignIface, GenesisIface, GlobalIface, Iface, OwnedIface, Req, TransitionIface, VerNo,
};
use crate::interface::contract::OutpointFilter;
use crate::interface::{ArgSpec, ContractIface, FungibleAllocation};
use crate::stl::{Amount, ContractData, DivisibleAssetSpec, StandardTypes};
use std::collections::HashMap;

pub const LIB_NAME_RGB_E20: &str = "RGB-E20";

const SUPPLY_MISMATCH: u8 = 1;
const NON_EQUAL_AMOUNTS: u8 = 2;
const INVALID_PROOF: u8 = 3;
const INSUFFICIENT_RESERVES: u8 = 4;
const INSUFFICIENT_COVERAGE: u8 = 5;
const ISSUE_EXCEEDS_ALLOWANCE: u8 = 6;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_E20, tags = repr, into_u8, try_from_u8)]
#[repr(u8)]
pub enum Error {
    #[strict_type(dumb)]
    SupplyMismatch = SUPPLY_MISMATCH,
    NonEqualAmounts = NON_EQUAL_AMOUNTS,
    InvalidProof = INVALID_PROOF,
    InsufficientReserves = INSUFFICIENT_RESERVES,
    InsufficientCoverage = INSUFFICIENT_COVERAGE,
    IssueExceedsAllowance = ISSUE_EXCEEDS_ALLOWANCE,
}

type Balances = HashMap<Principal, u64>;
type Allowances = HashMap<Principal, HashMap<Principal, u64>>;

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct RgbE20 {
    name: String,
    symbol: String,
    owner: Account,
    total_supply: U256,
}

pub fn rgb_e20() -> Iface {
    Iface {
        version: VerNo::V1,
        name: tn!("RGB-E20"),
        global_state: tiny_bmap! {
            fname!("name") => GlobalIface::required(types.get("RGBContract.Name")),
            fname!("Symbol") => GlobalIface::required(types.get("RGBContract.Data")),
            fname!("total_supply") => GlobalIface::required(types.get("RGBContract.Amount")),
            fname!("decimals") => GlobalIface::one_or_many(types.get("RGBContract.Amount")),
        },
        genesis: GenesisIface {
            metadata: Some(types.get("RGBContract")),
            global: tiny_bmap! {
                fname!("init") => ArgSpec::required(),
            },
            assignments: none!(),
            valencies: none!(),
            errors: tiny_bset! {
                INSUFFICIENT_RESERVES
            },
        },
        transitions: tiny_bmap! {
            tn!("Transfer") => TransitionIface {
                optional: false,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("to") => ArgSpec::from_non_empty("account"),
                    fname!("value") => ArgSpec::from_non_empty("account"),
                },
                valencies: none!(),
                errors: tiny_bset! {
                    NON_EQUAL_AMOUNTS
                },
                default_assignment: Some(fname!("beneficiary")),
            },
            tn!("Issue") => TransitionIface {
                optional: true,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("used") => ArgSpec::from_non_empty("inflationAllowance"),
                },
                valencies: none!(),
                errors: tiny_bset! {
                    SUPPLY_MISMATCH,
                    INVALID_PROOF,
                    ISSUE_EXCEEDS_ALLOWANCE,
                    INSUFFICIENT_RESERVES
                },
                default_assignment: Some(fname!("beneficiary")),
            },
            tn!("TransferFrom") => TransitionIface {
                optional: true,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("from") => ArgSpec::from_required("account"),
                    fname!("to") => ArgSpec::from_required("account"),
                    fname!("value") => ArgSpec::from_required("amount"),
                },
                errors: none!(),
                default_assignment: Some(fname!("transferFrom")),
            },
            tn!("Burn") => TransitionIface {
                optional: true,
                metadata: None,
                globals: tiny_bmap! {
                    fname!("burnedSupply") => ArgSpec::required(),
                },
                inputs: tiny_bmap! {
                    fname!("used") => ArgSpec::from_required("burnRight"),
                },
                errors: tiny_bset! {
                    SUPPLY_MISMATCH,
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: None,
            },
            tn!("Approve") => TransitionIface {
                optional: true,
                metadata: none!(),
                inputs: tiny_bmap! {
                    fname!("spender") => ArgSpec::from_required("amount"),
                    fname!("value") => ArgSpec::from_required("amount"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: None,
            },
            tn!("Mint") => TransitionIface {
                optional: true,
                metadata: None,
                globals: none!()
                inputs: tiny_bmap! {
                    fname!("to") => ArgSpec::from_required("account"),
                    fname!("value") => ArgSpec::from_required("amount"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: None,
            },
            tn!("BalanceOf") => TransitionIface {
                optional: true,
                metadata: none!(),
                globals: none!()
                inputs: tiny_bmap! {
                    fname!("account") => ArgSpec::from_required("Account"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: Some(fname!("balance_of")),
            },
            tn!("Allowance") => TransitionIface {
                optional: true,
                metadata: none!(),
                globals: none!()
                inputs: tiny_bmap! {
                    fname!("owner") => ArgSpec::from_required("account"),
                    fname!("spender") => ArgSpec::from_required("account"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: None,
            },
        },
        extensions: none!(),
        error_type: types.get("RGB-E20.Error"),
        default_operation: Some(tn!("Transfer")),
    }
}

pub trait RgbE20Iface {
    //Genesis
    fn init(&self, name: &str, symbol: &str);
    //Transaction
    fn transfer(to: Account, value: u64) -> bool;
    fn transfer_from(from: Account, to: Account, value: u64) -> bool;
    fn approve(spender: Account, value: u64) -> bool;
    fn mint(to: Account, value: u64) -> bool;
    fn burn(from: Account, value: u64) -> bool;
    fn balance_of(account: Account) -> u64;
    fn allowance(owner: Account, spender: Account) -> u64;
    //Global
    fn name() -> String;
    fn symbol() -> String;
    fn decimals() -> u64;
    fn total_supply() -> u64;
}
