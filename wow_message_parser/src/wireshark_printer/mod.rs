mod printer;
mod types;

use crate::file_utils::overwrite_if_not_same_contents;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArrayType, Endianness, IntegerType};
use crate::rust_printer::Writer;
use crate::wireshark_printer::printer::{
    print_enums, print_int_declarations, print_parser, print_register_info,
};
use crate::{Object, Objects, Tags};
use heck::{ShoutySnakeCase, SnakeCase, TitleCase};
use std::fs::read_to_string;
use std::path::Path;

pub fn print_wireshark(o: &Objects, path: &str) {
    let w = types::get_wireshark_object(o);
    let imports = print_int_declarations(&w);
    let enums = print_enums(&w);
    let register = print_register_info(&w);
    let (parser, variables) = print_parser(o, &w);

    apply_to_file(imports, enums, register, parser, path, variables);
}

fn apply_to_file(
    imports: Writer,
    enums: Writer,
    register: Writer,
    parser: Writer,
    path: &str,
    variables: Writer,
) {
    let path = Path::new(path);
    let s = read_to_string(&path).unwrap();

    const HF_START: &str = "/* AUTOGENERATED_START_HF */";
    const HF_END: &str = "/* AUTOGENERATED_END_HF */";
    let s = insert_between(&s, HF_START, HF_END, imports.inner());

    const ENUM_START: &str = "/* AUTOGENERATED_START_ENUM */";
    const ENUM_END: &str = "/* AUTOGENERATED_END_ENUM */";
    let s = insert_between(&s, ENUM_START, ENUM_END, enums.inner());

    const REGISTER_START: &str = "/* AUTOGENERATED_START_REGISTER */";
    const REGISTER_END: &str = "/* AUTOGENERATED_END_REGISTER */";
    let s = insert_between(&s, REGISTER_START, REGISTER_END, register.inner());

    const VARIABLES_START: &str = "/* AUTOGENERATED_START_VARIABLES */";
    const VARIABLES_END: &str = "/* AUTOGENERATED_END_VARIABLES */";
    let s = insert_between(&s, VARIABLES_START, VARIABLES_END, variables.inner());

    const PARSER_START: &str = "/* AUTOGENERATED_START_PARSER */";
    const PARSER_END: &str = "/* AUTOGENERATED_END_PARSER */";
    let s = insert_between(&s, PARSER_START, PARSER_END, parser.inner());

    overwrite_if_not_same_contents(&s, path);
}

fn insert_between(contents: &str, start: &str, end: &str, replace_with: &str) -> String {
    let (before, mid) = contents.split_once(start).unwrap();
    let (_, after) = mid.split_once(end).unwrap();

    let mut s = before.to_string();
    s += start;
    s += "\n";
    s += replace_with;
    s += end;
    s += after;

    s
}

fn is_client_name(name: &str) -> bool {
    name.contains("_Client")
}

fn is_server_name(name: &str) -> bool {
    name.contains("_Server")
}

fn server_to_client_name(name: &str) -> String {
    name.replace("_Server", "_Client")
}

fn clean_opcode_name(name: &str) -> String {
    name.replace("_Server", "").replace("_Client", "")
}

fn ui_name(name: &str) -> String {
    name.replace('_', ".")
}

fn enum_name(name: &str) -> String {
    format!("e_{}", name.to_snake_case())
}

fn enumerator_name(enum_name: &str, name: &str) -> String {
    format!(
        "{}_{}",
        enum_name.to_shouty_snake_case(),
        name.to_shouty_snake_case()
    )
}

fn enum_strings(name: &str) -> String {
    format!("{}_strings", enum_name(name))
}

fn pretty_name(name: &str) -> String {
    name.to_title_case()
}

fn name_to_hf(name: &str, ty: &Type, tags: &Tags, o: &Objects) -> String {
    let mut name = match ty {
        Type::Identifier { s, .. } => {
            let e = o.get_object(s, tags);

            match e {
                Object::Container(_) => name.to_string(),
                Object::Enum(e) | Object::Flag(e) => e.name().to_snake_case(),
            }
        }
        _ => name.to_string(),
    };

    let pos = name.chars().enumerate().find(|(_, a)| a.is_numeric());

    if let Some((i, _)) = pos {
        name = name[..i].to_string();
    }

    if name == "character" {
        match ty {
            Type::PackedGuid | Type::Guid => {
                name += "_guid";
            }
            Type::CString | Type::SizedCString | Type::String { .. } => {
                name += "_name";
            }
            _ => unreachable!(),
        }
    } else if name == "unknown" {
        match ty {
            Type::Integer(_) => {
                name += "_int";
            }
            Type::FloatingPoint(_) => name += "_float",
            Type::Array(array) => match array.ty() {
                ArrayType::Integer(_) => {
                    name += "_bytes";
                }
                _ => unreachable!(),
            },
            Type::PackedGuid | Type::Guid => {
                name += "_guid";
            }
            _ => panic!("unknown got type '{:#?}'", ty),
        }
    } else if name == "emote" {
        match ty {
            Type::Integer(_) => {
                name += "_int";
            }
            Type::Identifier { s, .. } => {
                let e = o.get_object(s, tags);
                match e {
                    Object::Flag(_) | Object::Container(_) => unreachable!(),
                    Object::Enum(_) => {
                        name += "_enum";
                    }
                }
            }
            _ => unreachable!(),
        }
    } else if name.starts_with("position")
        && matches!(ty, Type::Integer(IntegerType::U16(Endianness::Little)))
    {
        name += "_int";
    }

    format!("hf_woww_{}", name)
}
