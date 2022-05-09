use crate::container::{
    Conditional, Container, ContainerType, Equation, IfStatement, OptionalStatement, StructMember,
    StructMemberDefinition,
};
use crate::impl_features::{get_impl_features_for_container, Feature};
use crate::ir_printer::{IrEndianness, IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::ty::Type;
use crate::parser::types::{
    Array, ArraySize, ArrayType, FloatingPointType, VerifiedContainerValue,
};
use crate::test_case::{TestCase, TestCaseMember, TestValue};
use serde::Serialize;

pub fn containers_to_ir(containers: &[&Container]) -> Vec<IrContainer> {
    containers.iter().map(|a| container_to_ir(a)).collect()
}

fn container_to_ir(e: &Container) -> IrContainer {
    let members = e.fields().iter().map(|a| a.into()).collect();

    let tests = e.tests().iter().map(|a| a.into()).collect();

    IrContainer {
        name: e.name().to_string(),
        object_type: e.container_type().into(),
        constant: e.is_constant_sized(),
        members,
        tags: IrTags::from_tags(e.tags()),
        tests,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line(),
        },
        only_has_io_error: e.only_has_io_errors(),
        features: get_impl_features_for_container(e).to_array(),
    }
}

#[derive(Debug, Serialize)]
pub enum IrContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
    SMsg(u16),
}

impl From<ContainerType> for IrContainerType {
    fn from(v: ContainerType) -> Self {
        match v {
            ContainerType::Struct => Self::Struct,
            ContainerType::CLogin(o) => Self::CLogin(o),
            ContainerType::SLogin(o) => Self::SLogin(o),
            ContainerType::Msg(o) => Self::Msg(o),
            ContainerType::CMsg(o) => Self::CMsg(o),
            ContainerType::SMsg(o) => Self::SMsg(o),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrContainer {
    name: String,
    object_type: IrContainerType,
    constant: bool,
    members: Vec<IrStructMember>,
    tags: IrTags,
    tests: Vec<IrTestCase>,
    file_info: IrFileInfo,
    only_has_io_error: bool,
    features: Vec<Feature>,
}

impl IrContainer {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize)]
pub struct IrOptionalStatement {
    name: String,
    members: Vec<IrStructMember>,
    tags: IrTags,
}

impl From<&OptionalStatement> for IrOptionalStatement {
    fn from(v: &OptionalStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            name: v.name().to_string(),
            members,
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrStructMember {
    Definition(IrStructMemberDefinition),
    IfStatement(IrIfStatement),
    Optional(IrOptionalStatement),
}

impl From<&StructMember> for IrStructMember {
    fn from(v: &StructMember) -> Self {
        match v {
            StructMember::Definition(d) => Self::Definition(d.into()),
            StructMember::IfStatement(statement) => Self::IfStatement(statement.into()),
            StructMember::OptionalStatement(optional) => Self::Optional(optional.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrEquation {
    Equals { value: String },
    NotEquals { value: String },
    BitwiseAnd { value: String },
}

impl From<&Equation> for IrEquation {
    fn from(v: &Equation) -> Self {
        match v {
            Equation::Equals { value } => IrEquation::Equals {
                value: value.to_string(),
            },
            Equation::NotEquals { value } => IrEquation::NotEquals {
                value: value.to_string(),
            },
            Equation::BitwiseAnd { value } => IrEquation::BitwiseAnd {
                value: value.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrConditional {
    variable_name: String,
    equations: Vec<IrEquation>,
}

impl From<Conditional> for IrConditional {
    fn from(v: Conditional) -> Self {
        let equations = v.equations().iter().map(|a| a.into()).collect();

        Self {
            variable_name: v.variable_name().to_string(),
            equations,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrIfStatement {
    pub conditional: IrConditional,
    members: Vec<IrStructMember>,
    else_ifs: Vec<IrIfStatement>,
    else_statement_members: Vec<IrStructMember>,
    original_ty: IrType,
}

impl From<&IfStatement> for IrIfStatement {
    fn from(v: &IfStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();
        let else_ifs = v.else_ifs().iter().map(|a| a.into()).collect();

        let else_statement_members = v.else_members().iter().map(|a| a.into()).collect();

        Self {
            conditional: v.conditional.clone().into(),
            members,
            else_ifs,
            else_statement_members,
            original_ty: v.original_ty().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrStructMemberDefinition {
    name: String,
    member_type: IrType,
    constant_value: Option<IrIntegerEnumValue>,
    used_as_size_in: Option<String>,
    used_in_if: bool,
    tags: IrTags,
}

impl From<&StructMemberDefinition> for IrStructMemberDefinition {
    fn from(v: &StructMemberDefinition) -> Self {
        Self {
            name: v.name().to_string(),
            member_type: v.ty().into(),
            constant_value: v.verified_value().as_ref().map(|a| a.into()),
            used_as_size_in: v.used_as_size_in().clone(),
            used_in_if: v.used_in_if(),
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrType {
    Integer(IrIntegerType),
    PackedGuid,
    Guid,
    FloatingPoint(IrFloatingPointType),
    CString,
    String {
        length: String,
    },
    Array(IrArray),
    Identifier {
        s: String,
        upcast: Option<IrIntegerType>,
    },
    UpdateMask,
    AuraMask,
}

impl From<&Type> for IrType {
    fn from(v: &Type) -> Self {
        match v {
            Type::Integer(i) => Self::Integer(i.into()),
            Type::PackedGuid => Self::PackedGuid,
            Type::Guid => Self::Guid,
            Type::FloatingPoint(f) => Self::FloatingPoint(f.into()),
            Type::CString => Self::CString,
            Type::String { length } => Self::String {
                length: length.to_string(),
            },
            Type::UpdateMask => Self::UpdateMask,
            Type::AuraMask => Self::AuraMask,
            Type::Array(array) => Self::Array(array.into()),
            Type::Identifier { s, upcast } => Self::Identifier {
                s: s.to_string(),
                upcast: upcast.map(|a| (&a).into()),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrArray {
    inner: IrArrayType,
    size: IrArraySize,
}

impl From<&Array> for IrArray {
    fn from(v: &Array) -> Self {
        Self {
            inner: v.ty().into(),
            size: v.size().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrArrayType {
    Integer(IrIntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl From<&ArrayType> for IrArrayType {
    fn from(v: &ArrayType) -> Self {
        match v {
            ArrayType::Integer(i) => Self::Integer(i.into()),
            ArrayType::Complex(f) => Self::Complex(f.into()),
            ArrayType::CString => Self::CString,
            ArrayType::Guid => Self::Guid,
            ArrayType::PackedGuid => Self::PackedGuid,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrArraySize {
    Fixed(i64),
    Variable(String),
    Endless,
}

impl From<ArraySize> for IrArraySize {
    fn from(v: ArraySize) -> Self {
        match v {
            ArraySize::Fixed(s) => Self::Fixed(s),
            ArraySize::Variable(s) => Self::Variable(s.to_string()),
            ArraySize::Endless => Self::Endless,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrFloatingPointType {
    F32(IrEndianness),
    F64(IrEndianness),
}

impl From<&FloatingPointType> for IrFloatingPointType {
    fn from(v: &FloatingPointType) -> Self {
        match v {
            FloatingPointType::F32(f) => Self::F32(f.into()),
            FloatingPointType::F64(f) => Self::F64(f.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrIntegerEnumValue {
    value: u64,
    original_string: String,
}

impl From<&VerifiedContainerValue> for IrIntegerEnumValue {
    fn from(v: &VerifiedContainerValue) -> Self {
        Self {
            value: v.value(),
            original_string: v.original_string().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTestCase {
    subject: String,
    members: Vec<IrTestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: IrTags,
    file_info: IrFileInfo,
}

impl From<&TestCase> for IrTestCase {
    fn from(v: &TestCase) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            subject: v.subject().to_string(),
            members,
            raw_bytes: v.raw_bytes().to_vec(),
            tags: IrTags::from_tags(v.tags()),
            file_info: IrFileInfo {
                file_name: v.file_info().name().to_string(),
                start_position: v.file_info().start_line(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTestCaseMember {
    variable_name: String,
    verified_value: IrTestValue,
    tags: IrTags,
}

impl From<&TestCaseMember> for IrTestCaseMember {
    fn from(v: &TestCaseMember) -> Self {
        Self {
            variable_name: v.name().to_string(),
            verified_value: v.value().into(),
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrTestValue {
    Number(IrIntegerEnumValue),
    Guid(IrIntegerEnumValue),
    FloatingNumber {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<usize>,
        size: IrArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(IrIntegerEnumValue),
    SubObject {
        ty_name: String,
        members: Vec<IrTestCaseMember>,
    },
    ArrayOfSubObject(String, Vec<Vec<IrTestCaseMember>>),
}

impl From<&TestValue> for IrTestValue {
    fn from(v: &TestValue) -> Self {
        match v {
            TestValue::Number(i) => Self::Number(i.into()),
            TestValue::Guid(i) => Self::Guid(i.into()),
            TestValue::FloatingNumber {
                value,
                original_string,
            } => Self::FloatingNumber {
                value: *value,
                original_string: original_string.to_string(),
            },
            TestValue::Array { values, size } => Self::Array {
                values: values.to_vec(),
                size: size.clone().into(),
            },
            TestValue::String(s) => Self::String(s.to_string()),
            TestValue::Flag(f) => Self::Flag(f.to_vec()),
            TestValue::Enum(e) => Self::Enum(e.into()),
            TestValue::SubObject { ty_name, members } => Self::SubObject {
                ty_name: ty_name.to_string(),
                members: members.iter().map(|a| a.into()).collect(),
            },
            TestValue::ArrayOfSubObject(s, t) => Self::ArrayOfSubObject(
                s.to_string(),
                t.iter()
                    .map(|a| a.iter().map(|a| a.into()).collect::<Vec<_>>())
                    .collect(),
            ),
        }
    }
}
