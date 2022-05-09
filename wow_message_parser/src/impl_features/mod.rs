use crate::container::{Container, ContainerType, StructMember, StructMemberDefinition};
use crate::parser::enumerator::Definer;
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType};
use crate::rust_printer::DefinerType;
use crate::CONTAINER_SELF_SIZE_FIELD;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Feature {
    Struct,
    CLogin,
    SLogin,
    SMsg,
    CMsg,
    Msg,

    NestedStructTypes,

    ConstantValue,
    ManualSizeField,

    SimpleBuiltInTypes,
    ComplexBuiltInTypes,
    PackedGuidTypes,
    UpdateMaskTypes,
    AuraMaskTypes,

    EmptyContainer,

    SimpleArrays,
    ComplexArrays,

    FixedArrays,
    VariableArrays,
    EndlessArrays,

    SubObjects,

    IfEnum,
    IfNotEnum,
    IfElseEnum,
    NotElseEnum,
    ElseEnum,

    IfFlag,
    IfElseFlag,
    ElseFlag,

    IfMultiCondition,

    Optional,

    Enum,
    EnumSelf,
    EnumUpcast,
    Flag,
    SimpleDefinerTypes,

    WorldVersion,
    LoginVersion,
}

pub struct ImplFeatures {
    inner: Vec<Feature>,
}

impl ImplFeatures {
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(16),
        }
    }

    pub fn add(&mut self, f: Feature) {
        if !self.inner.contains(&f) {
            self.inner.push(f);
        }
    }
}

pub fn get_impl_features_for_definer(e: &Definer) -> ImplFeatures {
    let mut f = ImplFeatures::new();
    match e.definer_ty() {
        DefinerType::Enum => f.add(Feature::Enum),
        DefinerType::Flag => f.add(Feature::Flag),
    };

    if let Some(_) = e.self_value() {
        f.add(Feature::EnumSelf);
    }

    add_version(e.tags(), &mut f);

    f
}

pub fn get_impl_features_for_container(e: &Container) -> ImplFeatures {
    let mut f = ImplFeatures::new();

    match e.container_type() {
        ContainerType::Struct => f.add(Feature::Struct),
        ContainerType::CLogin(_) => f.add(Feature::CLogin),
        ContainerType::SLogin(_) => f.add(Feature::SLogin),
        ContainerType::Msg(_) => f.add(Feature::Msg),
        ContainerType::CMsg(_) => f.add(Feature::CMsg),
        ContainerType::SMsg(_) => f.add(Feature::SMsg),
    };

    if e.fields().len() == 0 {
        f.add(Feature::EmptyContainer);
    }

    for m in e.fields() {
        features_for_struct_member(&mut f, m);
    }

    add_version(e.tags(), &mut f);

    f
}

fn features_for_struct_member(f: &mut ImplFeatures, m: &StructMember) {
    match m {
        StructMember::Definition(d) => features_for_definition(f, d),
        StructMember::IfStatement(statement) => {
            let ty = statement.definer_type();
            f.add(match ty {
                DefinerType::Enum => {
                    if statement.is_not_enum() {
                        Feature::IfNotEnum
                    } else {
                        Feature::IfEnum
                    }
                }
                DefinerType::Flag => Feature::IfFlag,
            });

            if statement.conditional.equations().len() > 1 {
                f.add(Feature::IfMultiCondition);
            }

            for m in statement.members() {
                features_for_struct_member(f, m);
            }

            for elseif in statement.else_ifs() {
                f.add(match ty {
                    DefinerType::Enum => Feature::IfElseEnum,
                    DefinerType::Flag => Feature::IfElseFlag,
                });
                for m in elseif.members() {
                    features_for_struct_member(f, m);
                }
            }

            for m in &statement.else_statement_members {
                f.add(match ty {
                    DefinerType::Enum => {
                        if statement.is_not_enum() {
                            Feature::NotElseEnum
                        } else {
                            Feature::ElseEnum
                        }
                    }
                    DefinerType::Flag => Feature::ElseFlag,
                });

                features_for_struct_member(f, m);
            }
        }
        StructMember::OptionalStatement(optional) => {
            f.add(Feature::Optional);

            for m in optional.members() {
                features_for_struct_member(f, m);
            }
        }
    }
}

fn features_for_definition(f: &mut ImplFeatures, d: &StructMemberDefinition) {
    match d.ty() {
        Type::Integer(_) | Type::FloatingPoint(_) => {
            f.add(Feature::SimpleBuiltInTypes);

            if let Some(v) = d.verified_value() {
                if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                    f.add(Feature::ManualSizeField);
                }
                f.add(Feature::ConstantValue);
            }
        }
        Type::Guid | Type::CString | Type::String { .. } => f.add(Feature::ComplexBuiltInTypes),
        Type::UpdateMask => f.add(Feature::UpdateMaskTypes),
        Type::AuraMask => f.add(Feature::AuraMaskTypes),
        Type::PackedGuid => f.add(Feature::PackedGuidTypes),
        Type::Array(array) => {
            match array.size() {
                ArraySize::Fixed(_) => f.add(Feature::FixedArrays),
                ArraySize::Variable(_) => f.add(Feature::VariableArrays),
                ArraySize::Endless => f.add(Feature::EndlessArrays),
            }
            match array.ty() {
                ArrayType::Integer(_) => f.add(Feature::SimpleArrays),
                ArrayType::Complex(_)
                | ArrayType::CString
                | ArrayType::Guid
                | ArrayType::PackedGuid => f.add(Feature::ComplexArrays),
            }
        }
        Type::Identifier { upcast, .. } => {
            if let Some(_) = upcast {
                f.add(Feature::EnumUpcast);
            }

            f.add(Feature::SubObjects);
        }
    }
}

fn add_version(tags: &Tags, f: &mut ImplFeatures) {
    if !tags.versions().is_empty() {
        f.add(Feature::WorldVersion);
    } else {
        f.add(Feature::LoginVersion);
    }
}
