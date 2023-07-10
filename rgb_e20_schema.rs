/// RGB-E20 SCHEMA
use aluvm::library::{Lib, LibSite};
use rgbstd::interface::{rgb_e20,IfaceImpl, NamedField, NamedType, VerNo};
use rgbstd::schema::{
    FungibleType, GenesisSchema, GlobalStateSchema, Occurrences, Schema, Script, StateSchema,
    SubSchema, TransitionSchema,
};
use rgbstd::stl::StandardTypes;
use rgbstd::vm::{AluScript, ContractOp, EntryPoint, RgbIsa};
use strict_types::{SemId, Ty};

use crate::GS_TIMESTAMP;

const GS_NOMINAL: u16 = 20000;
const GS_CONTRACT: u16 = 20001;
const OS_ASSETS: u16 = 20000;
const TS_TRANSFER: u16 = 20000;

pub fn nia_schema() -> SubSchema {
    let types = StandardTypes::new();

    let code = [RgbIsa::Contract(ContractOp::PcVs(OS_ASSETS))];
    let alu_lib = Lib::assemble(&code).unwrap();
    let alu_id = alu_lib.id();
    Schema {
        ffv: zero!(),
        subset_of: None,
        type_system: types.type_system(),
        global_types: tiny_bmap! {
            GS_NAME => GlobalStateSchema::once(types.get("RGBContract.Text")),
            GS_SYMBOL => GlobalStateSchema::once(types.get("RGBContract.Text")),
            GS_DECIMALS => GlobalStateSchema::once(types.get("RGBContract.amount")),
            GS_TOTAL_SUPPLY => GlobalStateSchema::once(types.get("RGBContract.amount")),

        },
        owned_types: tiny_bmap! {
            OS_ASSETS => StateSchema::Fungible(FungibleType::Unsigned64Bit),
        },
        valency_types: none!(),
        genesis: GenesisSchema {
            metadata: Ty::<SemId>::UNIT.id(None),
            globals: tiny_bmap! {
                GS_INIT => Occurrences::Once,
            },
            assignments: tiny_bmap! {
                OS_ASSETS => Occurrences::OnceOrMore,
            },
            valencies: none!(),
        },
        extensions: none!(),
        transitions: tiny_bmap! {
            TS_TRANSFER => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                valencies: none!(),
            },
            TS_ISSUE => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: none!(),
                assignments: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                valencies: none!(),
            },
            TS_TRANSFER_FROM => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                valencies: none!(),
            },
            TS_BURN => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: none!(),
                assignments: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                valencies: none!(),
            },
            TS_APPROVE => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            },
             TS_MINT => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            },
            TS_BALANCE_OF => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            }
            TS_ALLOWANCE => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            }
        },
        script: Script::AluVM(AluScript {
            libs: confined_bmap! { alu_id => alu_lib },
            entry_points: confined_bmap! {
                EntryPoint::ValidateOwnedState(OS_ASSETS) => LibSite::with(0, alu_id)
            },
        }),
    }
}

pub fn nia_rgb_e20() -> IfaceImpl {
    let schema = nia_schema();
    let iface = rgb_e20();

    IfaceImpl {
        version: VerNo::V1,
        schema_id: schema.schema_id(),
        iface_id: iface.iface_id(),
        genesis_iface: tiny_bset! {
            NamedField::with(GS_INIT, fname!("init")),
        },
        global_state: tiny_bset! {
            NamedField::with(GS_NAME, fname!("name")),
            NamedField::with(GS_SYMBOL, fname!("symbol")),
            NamedField::with(GS_DECIMALS, fname!("decimals")),
            NamedField::with(GS_TOTAL_SUPPLY, fname!("total_supply")),
        },
        assignments: tiny_bset! {
            NamedField::with(OS_ASSETS, fname!("beneficiary")),
        },
        valencies: none!(),
        transitions: tiny_bset! {
            NamedType::with(TS_TRANSFER, tn!("transfer")),
            NamedType::with(TS_ISSUE, tn!("issue")),
            NamedType::with(TS_TRANSFER_FROM, tn!("transfer_from")),
            NamedType::with(TS_BURN, tn!("burn")),
            NamedType::with(TS_APPROVE, tn!("approve")),
            NamedType::with(TS_MINT, tn!("mint")),
            NamedType::with(TS_BALANCE_OF, tn!("balance_of")),
            NamedType::with(TS_ALLOWANCE, tn!("allowance")),
        },
        extensions: none!(),
    }
}
