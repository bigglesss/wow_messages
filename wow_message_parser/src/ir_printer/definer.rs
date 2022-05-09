use crate::container::DefinerUsage;
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::enumerator::{Definer, DefinerField};
use crate::rust_printer::DefinerType;
use core::convert::From;
use core::option::Option;
use serde::Serialize;

pub fn definers_to_ir(definers: &[Definer]) -> Vec<IrDefiner> {
    definers.iter().map(|a| definer_to_ir(a)).collect()
}

fn definer_to_ir(e: &Definer) -> IrDefiner {
    let fields = e
        .fields()
        .iter()
        .map(|a| -> IrDefinerField { a.into() })
        .collect();

    let objects_used_in = e
        .objects_used_in()
        .iter()
        .map(|a| (a.0.to_string(), a.1.into()))
        .collect();

    IrDefiner {
        name: e.name().to_string(),
        definer_ty: e.definer_ty().into(),
        enumerators: fields,
        self_value: None,
        integer_type: e.ty().into(),
        tags: IrTags::from_tags(e.tags()),
        objects_used_in,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line(),
        },
    }
}

#[derive(Debug, Serialize)]
enum IrDefinerType {
    Enum,
    Flag,
}

impl From<DefinerType> for IrDefinerType {
    fn from(v: DefinerType) -> Self {
        match v {
            DefinerType::Enum => IrDefinerType::Enum,
            DefinerType::Flag => IrDefinerType::Flag,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrDefinerUsage {
    UsedButNotInIf,
    InIf,
}

impl From<DefinerUsage> for IrDefinerUsage {
    fn from(v: DefinerUsage) -> Self {
        match v {
            DefinerUsage::Unused => unreachable!(),
            DefinerUsage::NotInIf => IrDefinerUsage::UsedButNotInIf,
            DefinerUsage::InIf => IrDefinerUsage::InIf,
        }
    }
}

#[derive(Debug, Serialize)]
struct IrSelfValueDefinerField {
    name: String,
    tags: IrTags,
}

#[derive(Debug, Serialize)]
pub struct IrDefinerField {
    name: String,
    value: IrDefinerValue,
    tags: IrTags,
}

impl From<&DefinerField> for IrDefinerField {
    fn from(a: &DefinerField) -> Self {
        IrDefinerField {
            name: a.name().to_string(),
            value: IrDefinerValue {
                int: a.value().int(),
                original: a.value().original().to_string(),
            },
            tags: IrTags::from_tags(a.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
struct IrDefinerValue {
    int: u64,
    original: String,
}

#[derive(Serialize, Debug)]
pub struct IrDefiner {
    name: String,
    definer_ty: IrDefinerType,
    enumerators: Vec<IrDefinerField>,
    self_value: Option<IrSelfValueDefinerField>,
    integer_type: IrIntegerType,
    tags: IrTags,
    objects_used_in: Vec<(String, IrDefinerUsage)>,
    file_info: IrFileInfo,
}
