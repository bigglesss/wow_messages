use crate::container::{Container, Equation, IfStatement, StructMember};
use crate::file_info::FileInfo;
use crate::parser::enumerator::DefinerValue;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::{Array, FloatingPointType, IntegerType, ObjectType};
use crate::test_case::TestCase;

#[derive(Debug, Clone)]
pub struct RustMember {
    name: String,
    ty: RustType,

    tags: Tags,
}

#[derive(Debug, Clone)]
pub struct RustEnumerator {
    name: String,
    value: DefinerValue,
    members: Vec<RustMember>,
}

#[derive(Debug, Clone)]
pub enum RustType {
    Integer(IntegerType),
    Floating(FloatingPointType),
    BuiltIn(String),
    String,
    Array(Array),
    Enum {
        enumerators: Vec<RustEnumerator>,
        upcast: Option<IntegerType>,
    },
    Flag {
        enumerators: Vec<RustEnumerator>,
    },
    Struct,
}

#[derive(Debug, Clone)]
pub struct RustOptional {
    name: String,
    members: Vec<RustMember>,
}

#[derive(Debug, Clone)]
pub struct RustObject {
    name: String,
    members: Vec<RustMember>,
    optional: Option<RustOptional>,

    tests: Vec<TestCase>,

    tags: Tags,
    file_info: FileInfo,
}

pub fn create_if_statement(
    statement: &IfStatement,
    ty_name: &str,
    tags: &Tags,
    o: &Objects,
    current_scope: &mut Vec<RustMember>,
    parent_scope: &mut Vec<RustMember>,
) {
    let mut if_enumerators = Vec::new();
    let mut not_enumerators = Vec::new();

    for i in statement.get_conditional().equations() {
        match i {
            Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                if_enumerators.push(value)
            }
            Equation::NotEquals { value } => not_enumerators.push(value),
        }
    }

    let mut if_enumerator_members = Vec::new();
    for m in statement.members() {
        create_struct_member(
            m,
            ty_name,
            tags,
            o,
            &mut if_enumerator_members,
            current_scope,
            &mut None,
        );
    }

    let mut else_enumerator_members = Vec::new();
    for m in &statement.else_statement_members {
        create_struct_member(
            m,
            ty_name,
            tags,
            o,
            &mut else_enumerator_members,
            current_scope,
            &mut None,
        );
    }

    let subject = current_scope
        .iter_mut()
        .find(|a| format!("{}{}", ty_name, statement.name()) == a.name);
    let subject = match subject {
        None => parent_scope
            .iter_mut()
            .find(|a| format!("{}{}", ty_name, statement.name()) == a.name)
            .unwrap(),
        Some(s) => s,
    };

    let enums = match &mut subject.ty {
        RustType::Enum { enumerators, .. } => enumerators,
        RustType::Flag { enumerators } => enumerators,
        _ => panic!(),
    };

    for i in &if_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name == i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for i in &not_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name != i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for i in &if_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name != i)
            .unwrap()
            .members
            .append(&mut else_enumerator_members.clone());
    }

    for i in &not_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name == i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for else_if in statement.else_ifs() {
        create_if_statement(else_if, ty_name, tags, o, current_scope, parent_scope);
    }
}

pub fn create_struct_member(
    m: &StructMember,
    ty_name: &str,
    tags: &Tags,
    o: &Objects,
    current_scope: &mut Vec<RustMember>,
    parent_scope: &mut Vec<RustMember>,
    optional: &mut Option<RustOptional>,
) {
    match m {
        StructMember::Definition(d) => {
            let ty = match d.ty() {
                Type::Integer(i) => {
                    if let Some(_) = d.used_as_size_in() {
                        return;
                    }
                    if let Some(_) = d.verified_value() {
                        return;
                    }
                    RustType::Integer(i.clone())
                }
                Type::Guid | Type::PackedGuid => RustType::BuiltIn("Guid".to_string()),
                Type::FloatingPoint(f) => RustType::Floating(f.clone()),
                Type::CString | Type::String { .. } => RustType::String,
                Type::Array(array) => RustType::Array(array.clone()),
                Type::Identifier { s, upcast } => {
                    let add_types = || -> Vec<RustEnumerator> {
                        let mut enumerators = Vec::new();
                        let definer = o.get_definer(s, tags);

                        for field in definer.fields() {
                            enumerators.push(RustEnumerator {
                                name: field.name().to_string(),
                                value: field.value().clone(),
                                members: vec![],
                            });
                        }
                        enumerators
                    };
                    match o.get_object_type_of(s, tags) {
                        ObjectType::Enum => {
                            let enumerators = add_types();

                            RustType::Enum {
                                enumerators,
                                upcast: upcast.clone(),
                            }
                        }
                        ObjectType::Flag => {
                            let enumerators = add_types();

                            RustType::Flag { enumerators }
                        }
                        ObjectType::Struct => RustType::Struct,
                        ObjectType::CLogin | ObjectType::SLogin => {
                            panic!("object contains message type")
                        }
                    }
                }
                Type::UpdateMask => RustType::BuiltIn("UpdateMask".to_string()),
                Type::AuraMask => RustType::BuiltIn("AuraMask".to_string()),
            };

            let name = d.name().to_string();
            let name = match ty {
                RustType::Enum { .. } | RustType::Flag { .. } => format!("{}{}", ty_name, name),
                _ => name,
            };

            current_scope.push(RustMember {
                name,
                ty,
                tags: tags.clone(),
            });
        }
        StructMember::IfStatement(statement) => {
            create_if_statement(statement, ty_name, tags, o, current_scope, parent_scope);
        }
        StructMember::OptionalStatement(option) => {
            let mut members = Vec::new();

            for i in option.members() {
                create_struct_member(i, ty_name, tags, o, &mut members, current_scope, &mut None);
            }

            *optional = Some(RustOptional {
                name: option.name().to_string(),
                members,
            });
        }
    }
}

pub fn create_rust_object(e: &Container, o: &Objects) -> RustObject {
    let mut v = Vec::new();
    let mut optional = None;

    for m in e.fields() {
        create_struct_member(m, e.name(), e.tags(), o, &mut v, &mut vec![], &mut optional);
    }

    RustObject {
        name: e.name().to_string(),
        members: v,
        optional,
        tests: e.tests().to_vec(),
        tags: e.tags().clone(),
        file_info: e.file_info().clone(),
    }
}
