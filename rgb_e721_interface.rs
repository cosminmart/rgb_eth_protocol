//RGB-E721
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

pub const LIB_NAME_RGB_E721: &str = "RGB-E721";

const SUPPLY_MISMATCH: u8 = 1;
const TOKEN_NOT_EXIST: u8 = 2;
const INVALID_PROOF: u8 = 3;
const INSUFFICIENT_RESERVES: u8 = 4;
const ISSUE_EXCEEDS_ALLOWANCE: u8 = 5;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_E721, tags = repr, into_u8, try_from_u8)]
#[repr(u8)]
pub enum Error {
    #[strict_type(dumb)]
    SupplyMismatch = SUPPLY_MISMATCH,
    NonEqualAmounts = TOKEN_NOT_EXIST,
    InvalidProof = INVALID_PROOF,
    InsufficientReserves = INSUFFICIENT_RESERVES,
    IssueExceedsAllowance = ISSUE_EXCEEDS_ALLOWANCE,
}

type Balances = HashMap<Account, u64>;
type Allowances = HashMap<Account, HashMap<Account, u64>>;
type Owner = HashMap<Account, u64>;


#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct RgbE721 {
    name: String,
    symbol: String,
    owner: Account,
    total_supply: U256,
    base_uri: Option<String>
}

pub fn rgb_e721() -> Iface {
    Iface {
        version: VerNo::V1,
        name: tn!("RGB-E721"),
        global_state: tiny_bmap! {
            fname!("name") => GlobalIface::required(types.get("RGBContract.Name")),
            fname!("Symbol") => GlobalIface::required(types.get("RGBContract.Data")),
            fname!("total_supply") => GlobalIface::required(types.get("RGBContract.Amount")),
            fname!("token_by_index") => GlobalIface::one_or_many(types.get("RGBContract.Amount")),
            fname!("token_of_owner_by_index") => GlobalIface::one_or_many(types.get("RGBContract.Amount")),
        },
        genesis: GenesisIface {
            metadata: Some(types.get("RGBContract.Meta")),
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
             tn!("OwnerOf") => TransitionIface {
                optional: false,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("token_id") => ArgSpec::from_non_empty("Index"),
                },
                valencies: none!(),
                errors: tiny_bset! {
                    NON_EQUAL_AMOUNTS
                },
                default_assignment: Some(fname!("beneficiary")),
            },
            tn!("SafeTransferFrom") => TransitionIface {
                optional: false,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("from") => ArgSpec::from_non_empty("account"),
                    fname!("to") => ArgSpec::from_non_empty("account"),
                    fname!("token_id") => ArgSpec::from_non_empty("Index"),
                },
                valencies: none!(),
                errors: tiny_bset! {
                    NON_EQUAL_AMOUNTS
                },
                default_assignment: Some(fname!("safe_transfer")),
            },
            tn!("TransferFrom") => TransitionIface {
                optional: true,
                metadata: none!(),
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("from") => ArgSpec::from_non_empty("Account"),
                    fname!("to") => ArgSpec::from_non_empty("Account"),
                    fname!("token_id") => ArgSpec::from_non_empty("Index"),

                },
                valencies: none!(),
                errors: tiny_bset! {
                    SUPPLY_MISMATCH,
                    INVALID_PROOF,
                    ISSUE_EXCEEDS_ALLOWANCE,
                    INSUFFICIENT_RESERVES
                },
                default_assignment: Some(fname!("transfer_from")),
            },
            tn!("Approve") => TransitionIface {
                optional: true,
                metadata: None,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("approved") => ArgSpec::from_required("account"),
                    fname!("token_id") => ArgSpec::from_required("Index"),
                },
                errors: none!(),
                default_assignment: Some(fname!("approve")),
            },
            tn!("SetApprovalForAll") => TransitionIface {
                optional: true,
                metadata: None,,
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("operator") => ArgSpec::from_required("Account"),
                    fname!("approved") => ArgSpec::from_required("Bool"),
                },
                errors: tiny_bset! {
                    SUPPLY_MISMATCH,
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: Some(fname!("set_approval_for_all")),
            },
            tn!("GetApproved") => TransitionIface {
                optional: true,
                metadata: none!(),
                inputs: tiny_bmap! {
                    fname!("token_id") => ArgSpec::from_required("Account"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: Some(fname!("get_approved")),
            },
            tn!("IsApprovedForAll") => TransitionIface {
                optional: true,
                metadata: none!(),
                globals: none!()
                inputs: tiny_bmap! {
                    fname!("owner") => ArgSpec::from_required("account"),
                    fname!("operator") => ArgSpec::from_required("account"),
                },
                errors: tiny_bset! {
                    INVALID_PROOF,
                    INSUFFICIENT_COVERAGE
                },
                default_assignment: Some(fname!("is_approved_for_all")),
            }
        },
        extensions: none!(),
        error_type: types.get("RGB-E721.Error"),
        default_operation: Some(tn!("Transfer")),
    }
}

pub trait RgbE721Iface {
    //Genesis
    fn init(&self, name: &str, symbol: &str);
    //Transaction
    fn owner_of(token_id: U256) -> Account;
    fn safe_transfer_from(from: Account,to: Account,token_id: U256);
    fn transfer_from(from: Account,to: Account,token_id: U256);
    fn approve(approved: Account,token_id: U256) -> Account;
    fn set_approval_for_all(operator: Account,approved: bool) -> Account;
    fn get_approved(token_id: U256) -> Account;
    fn is_approved_for_all(owner:Account,operator: Account) -> bool;
    //Global metadata
    fn name() -> String;
    fn symbol() -> String;
    fn token_uri(token_id:U256) -> String;
    fn total_supply() -> U256;
    fn token_by_index(index: U256) -> U256;
    fn token_of_owner_by_index(owner: Account,index: U256) -> U256;
}
