///RGB-E721 SCHEMA
use rgbstd::interface::{rgb_e721, rgb_e721_stl, IfaceImpl, NamedField, NamedType, VerNo};
use rgbstd::schema::{
    GenesisSchema, GlobalStateSchema, Occurrences, Schema, Script, StateSchema, SubSchema,
    TransitionSchema,
};
use rgbstd::stl::StandardTypes;
use rgbstd::vm::AluScript;
use strict_types::{SemId, Ty};

use crate::GS_TIMESTAMP;

const GS_NOMINAL: u16 = 72100;
const GS_CONTRACT: u16 = 72101;
const GS_TOKENS: u16 = 72102;
#[allow(dead_code)]
const GS_ENGRAVINGS: u16 = 72103;
const GS_ATTACH: u16 = 72104;
const OS_ASSET: u16 = 72100;
const TS_TRANSFER: u16 = 72100;

pub fn uda_schema() -> SubSchema {
    let types = StandardTypes::with(rgb721_stl());

    Schema {
        ffv: zero!(),
        subset_of: None,
        type_system: types.type_system(),
        global_types: tiny_bmap! {
           GS_NAME => GlobalStateSchema::once(types.get("RGBContract.Text")),
            GS_SYMBOL => GlobalStateSchema::once(types.get("RGBContract.Text")),
            GS_TOTAL_SUPPLY => GlobalStateSchema::once(types.get("RGBContract.amount")),
            GS_TOKEN_BY_INDEX => GlobalStateSchema::once(types.get("RGBContract.amount")),
            GS_TOKEN_OF_OWNER_BY_INDEX => GlobalStateSchema::once(types.get("RGBContract.amount")),

        },
        owned_types: tiny_bmap! {
            OS_ASSET => StateSchema::Structured(types.get("RGB_E721.Allocation")),
        },
        valency_types: none!(),
        genesis: GenesisSchema {
            metadata: Ty::<SemId>::UNIT.id(None),
            globals: tiny_bmap! {
                GS_INIT => Occurrences::Once,
            },
            assignments: tiny_bmap! {
                OS_ASSET => Occurrences::Once,
            },
            valencies: none!(),
        },
        extensions: none!(),
        transitions: tiny_bmap! {
            TS_OWNER_OF => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            },
            TS_SAFE_TRANSFER_FROM => TransitionSchema {
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
            TS_APPROVE => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            },
             TS_SET_APPROVED_FOR_ALL => TransitionSchema {
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
            TS_GET_APPROVED => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: none!(),
                valencies: none!(),
            }
            TS_IS_APPROVED_FOR_ALL => TransitionSchema {
                metadata: Ty::<SemId>::UNIT.id(None),
                globals: none!(),
                inputs: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                assignments: tiny_bmap! {
                    OS_ASSETS => Occurrences::OnceOrMore
                },
                valencies: none!(),
            }
        },
        script: Script::AluVM(AluScript {
            libs: none!(),
            entry_points: none!(),
        }),
    }
}

pub fn uda_rgb721() -> IfaceImpl {
    let schema = uda_schema();
    let iface = rgb721();

    IfaceImpl {
        version: VerNo::V1,
        schema_id: schema.schema_id(),
        iface_id: iface.iface_id(),
        global_state: tiny_bset! {
            NamedField::with(GS_NAME, fname!("name")),
            NamedField::with(GS_SYMBOL, fname!("symbol")),
            NamedField::with(GS_TOTAL_SUPPLY, fname!("total_supply")),
            NamedField::with(GS_TOKEN_BY_INDEX, fname!("token_by_index")),
            NamedField::with(GS_TOKEN_OF_OWNER_BY_INDEX, fname!("token_of_owner_by_index")),
        },
        assignments: tiny_bset! {
            NamedField::with(OS_ASSET, fname!("beneficiary")),
        },
        valencies: none!(),
        transitions: tiny_bset! {
            NamedType::with(TS_OWNER_OF, tn!("owner_of")),
            NamedType::with(TS_SAFE_TRANSFER_FROM, tn!("safeTransferFrom")),
            NamedType::with(TS_TRANSFER_FROM, tn!("transfer_from")),
            NamedType::with(TS_APPROVE, tn!("approve")),
            NamedType::with(TS_SET_APPROVED_FOR_ALL, tn!("set_approve_for_all")),
            NamedType::with(TS_GET_APPROVED, tn!("get_approved")),
            NamedType::with(TS_IS_APPROVED_FOR_ALL, tn!("is_approved_for_all")),
            extensions: none!(),
    }
}
