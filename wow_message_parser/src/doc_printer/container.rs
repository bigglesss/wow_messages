use crate::doc_printer::DocWriter;
use crate::parser::types::container::{
    Equation, IfStatement, StructMember, StructMemberDefinition,
};
use crate::parser::types::sizes::{BOOL_SIZE, DATETIME_SIZE};
use crate::parser::types::ty::Type;
use crate::parser::types::{Array, ArraySize, ArrayType, Endianness, IntegerType, ObjectType};
use crate::wowm_printer::get_struct_wowm_definition;
use crate::{doc_printer, Container, ContainerType, Objects, Tags};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;
use std::slice::Iter;

pub fn print_docs_for_container(e: &Container, o: &Objects) -> DocWriter {
    let mut s = DocWriter::new(e.name(), e.tags());

    doc_printer::common(&mut s, e.tags());

    s.wln("### Wowm Representation");
    s.newline();
    s.wln(format!(
        "Autogenerated from `wowm` file at {github_link}.",
        github_link = e.file_info().original_file_github_link()
    ));
    s.wln("```rust,ignore");
    s.w(get_struct_wowm_definition(e, ""));
    s.wln("```");

    print_container_header(&mut s, e);
    print_container_body(&mut s, e, o);

    print_container_examples(&mut s, e, o);

    s
}

fn print_container_example_array(
    s: &mut DocWriter,
    array: &Array,
    bytes: &mut Iter<u8>,
    values: &mut HashMap<String, isize>,
    o: &Objects,
    tags: &Tags,
    prefix: &str,
) {
    let size = match array.size() {
        ArraySize::Fixed(size) => size as usize,
        ArraySize::Variable(length) => *values.get(&length).unwrap() as usize,
        ArraySize::Endless => {
            let size = bytes.size_hint();
            assert_eq!(size.0, size.1.unwrap());
            size.0
        }
    };

    for i in 0..size {
        match array.ty() {
            ArrayType::Integer(t) => {
                let bytes = bytes.take(t.size() as usize);

                for b in bytes {
                    s.w_break_at(format!("{}, ", b));
                }
            }
            ArrayType::CString => {
                let mut b = bytes.next().unwrap();
                while *b != 0 {
                    s.w_break_at(format!("{}, ", b));
                    b = bytes.next().unwrap();
                }
                s.w_break_at(format!("{}, ", b));
            }
            ArrayType::Guid => {
                let bytes = bytes.take(core::mem::size_of::<u64>());

                for b in bytes {
                    s.w_break_at(format!("{}, ", b));
                }
            }
            ArrayType::PackedGuid => {
                let mask = bytes.next().unwrap();
                let bytes = bytes.take(mask.count_ones() as _);
                for b in bytes {
                    s.w_break_at(format!("{}, ", b));
                }
            }
            ArrayType::Complex(identifier) => {
                let type_of = o.get_object_type_of(identifier, tags);
                match type_of {
                    ObjectType::Struct => {}
                    _ => panic!(),
                }

                let c = o.get_container(identifier, tags);

                for m in c.fields() {
                    let prefix = format!("{}[{}].{}", prefix, i, c.name());
                    print_container_example_member(s, c, m, bytes, values, o, tags, &prefix);
                }
            }
        }
    }
}

fn print_container_example_definition(
    s: &mut DocWriter,
    d: &StructMemberDefinition,
    bytes: &mut Iter<u8>,
    values: &mut HashMap<String, isize>,
    o: &Objects,
    tags: &Tags,
    prefix: &str,
) {
    let comment = if !prefix.is_empty() {
        format!("// {}.{}: {}", prefix, d.name(), d.ty().str())
    } else {
        format!("// {}: {}", d.name(), d.ty().str())
    };

    match d.ty() {
        Type::Integer(t) => {
            let bytes = bytes.take(t.size() as usize).cloned().collect::<Vec<u8>>();

            let value = bytes.clone();
            let value = get_integer_value(t, value.as_slice());
            values.insert(d.name().to_string(), value);

            for b in bytes {
                s.w(format!("{}, ", b));
            }
        }
        Type::DateTime => {
            s.bytes(bytes.take(core::mem::size_of::<u32>()).into_iter());
        }
        Type::Bool => {
            s.bytes(bytes.take(core::mem::size_of::<u8>()).into_iter());
        }
        Type::Guid => {
            s.bytes(bytes.take(core::mem::size_of::<u64>()).into_iter());
        }
        Type::FloatingPoint(f) => {
            s.bytes(bytes.take(f.size() as usize).into_iter());
        }
        Type::PackedGuid => {
            let mask = bytes.next().unwrap();
            s.w(format!("{}, ", mask));
            let bytes = bytes.take(mask.count_ones() as _);
            s.bytes(bytes.into_iter());
        }
        Type::CString => {
            let mut b = bytes.next().unwrap();
            while *b != 0 {
                s.w(format!("{}, ", b));
                b = bytes.next().unwrap();
            }
            s.w(format!("{}, ", b));
        }
        Type::String { length } => {
            let length = if let Ok(length) = length.parse::<usize>() {
                length
            } else {
                *values.get(length).unwrap() as usize
            };
            s.bytes(bytes.take(length).into_iter());
        }
        Type::Array(array) => {
            print_container_example_array(s, array, bytes, values, o, tags, prefix);
        }
        Type::Identifier {
            s: identifier,
            upcast,
        } => {
            let type_of = o.get_object_type_of(identifier, tags);
            match type_of {
                ObjectType::Struct => {
                    let c = o.get_container(identifier, tags);

                    for m in c.fields() {
                        print_container_example_member(s, c, m, bytes, values, o, tags, c.name());
                    }

                    return;
                }
                ObjectType::Enum | ObjectType::Flag => {
                    let e = o.get_definer(identifier, tags);
                    let ty = if let Some(ty) = upcast { ty } else { e.ty() };

                    let bytes = bytes.take(ty.size() as usize).cloned().collect::<Vec<u8>>();

                    let value = get_integer_value(ty, bytes.as_slice());
                    values.insert(d.name().to_string(), value);
                    for b in bytes {
                        s.w(format!("{}, ", b));
                    }

                    let c = if type_of == ObjectType::Enum {
                        if let Some(value) = e.get_field_with_value(value) {
                            let name = value.name();
                            let value = value.value().original();
                            format!("{} {} ({})", comment, name, value)
                        } else {
                            "UNABLE TO FIND DEFINER VALUE PROBABLY FROM OTHER MISSING IMPLS"
                                .to_string()
                        }
                    } else if type_of == ObjectType::Flag {
                        let values = e.get_set_flag_fields(value);
                        let mut t = comment;
                        t.push(' ');

                        for (i, v) in values.iter().enumerate() {
                            if i != 0 {
                                t.push_str("| ");
                            } else {
                                t.push(' ');
                            }
                            t.push_str(v.name());
                        }

                        write!(t, " ({})", value).unwrap();
                        t
                    } else {
                        panic!()
                    };

                    s.wln(c);
                    return;
                }
                _ => panic!("unsupported"),
            }
        }
        Type::UpdateMask => {
            s.wln("// UpdateMask");
            let amount_of_blocks = bytes.next().unwrap();
            s.wln(format!("{}, // amount_of_blocks", amount_of_blocks));

            let blocks: Vec<&u8> = bytes.take(4 * *amount_of_blocks as usize).collect();
            let blocks = blocks.chunks(4);
            for (i, block) in blocks.clone().enumerate() {
                s.wln(format!(
                    "{}, {}, {}, {}, // Block {}",
                    block[0], block[1], block[2], block[3], i
                ));
            }
            let blocks = blocks.map(|a| u32::from_le_bytes([*a[0], *a[1], *a[2], *a[3]]));
            for block in blocks {
                for bit in 0..32 {
                    if (block & 1 << bit) != 0 {
                        s.bytes(bytes.take(4).into_iter());
                        s.wln_no_indent("// Item");
                    }
                }
            }
        }
        Type::AuraMask => panic!("AuraMask example"),
        Type::SizedCString => {
            let b = bytes.take(4).cloned().collect::<Vec<u8>>();

            for b in b {
                s.w(format!("{}, ", b));
            }
            s.w(" // SizedCString.length");

            let mut b = bytes.next().unwrap();
            while *b != 0 {
                s.w(format!("{}, ", b));
                b = bytes.next().unwrap();
            }
            s.w(format!("{}, ", b));
        }
    }
    s.wln(comment);
}

fn print_container_example_member(
    s: &mut DocWriter,
    e: &Container,
    m: &StructMember,
    bytes: &mut Iter<u8>,
    values: &mut HashMap<String, isize>,
    o: &Objects,
    tags: &Tags,
    prefix: &str,
) {
    match m {
        StructMember::Definition(d) => {
            print_container_example_definition(s, d, bytes, values, o, tags, prefix);
        }
        StructMember::IfStatement(statement) => {
            let enum_value = *values.get(statement.name()).unwrap();

            let definer_ty = match e.get_type_of_variable(statement.name()) {
                Type::Identifier { s: identifier, .. } => o.get_definer(&identifier, tags),
                _ => panic!(),
            };

            let statement_set = |statement: &IfStatement, enum_value: isize| {
                let mut set = false;
                for eq in statement.get_conditional().equations() {
                    match eq {
                        Equation::Equals { value } => {
                            let eq_value = definer_ty
                                .fields()
                                .iter()
                                .find(|a| a.name() == value)
                                .unwrap()
                                .value()
                                .int();

                            if eq_value == enum_value as u64 {
                                set = true;
                            }
                        }
                        Equation::BitwiseAnd { value } => {
                            let eq_value = definer_ty
                                .fields()
                                .iter()
                                .find(|a| a.name() == value)
                                .unwrap()
                                .value()
                                .int();

                            if (eq_value == 0 && enum_value == 0)
                                || (eq_value & enum_value as u64) != 0
                            {
                                set = true;
                            }
                        }
                        Equation::NotEquals { .. } => panic!(),
                    }
                }
                set
            };

            if statement_set(statement, enum_value) {
                for m in statement.members() {
                    print_container_example_member(s, e, m, bytes, values, o, tags, prefix);
                }
            } else if !statement.else_ifs().is_empty() {
                for elseif in statement.else_ifs() {
                    let value = *values.get(elseif.name()).unwrap();

                    if statement_set(elseif, value) {
                        for m in elseif.members() {
                            print_container_example_member(s, e, m, bytes, values, o, tags, prefix);
                        }
                    }
                }
            } else {
                for m in statement.else_members() {
                    print_container_example_member(s, e, m, bytes, values, o, tags, prefix);
                }
            }
        }
        StructMember::OptionalStatement(_) => {
            unimplemented!("UNIMPLEMENTED_DOC_OPTIONAL");
        }
    }
}

fn print_container_example_header(s: &mut DocWriter, e: &Container, bytes: &mut Iter<u8>) {
    match e.container_type() {
        ContainerType::CLogin(o) | ContainerType::SLogin(o) => {
            let bytes = bytes.take(core::mem::size_of::<u8>());
            s.bytes(bytes.into_iter());
            s.wln(format!("// opcode ({})", o));
            return;
        }
        ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
            let size = bytes.take(core::mem::size_of::<u16>());
            s.bytes(size.into_iter());
            s.wln("// size");
        }
        ContainerType::Msg(_) | ContainerType::Struct => panic!("struct/MSG not supported"),
    }

    let (opcode, o) = match e.container_type() {
        ContainerType::CMsg(o) => (bytes.take(core::mem::size_of::<u32>()), o),
        ContainerType::SMsg(o) => (bytes.take(core::mem::size_of::<u16>()), o),
        _ => panic!(),
    };
    s.bytes(opcode.into_iter());
    s.wln(format!("// opcode ({})", o));
}

fn print_container_examples(s: &mut DocWriter, e: &Container, o: &Objects) {
    if e.tests().is_empty() {
        return;
    }

    s.wln("### Examples");
    s.newline();

    for (i, t) in e.tests().iter().enumerate() {
        s.wln(format!("#### Example {}", i + 1));
        s.newline();

        if let Some(desc) = t.tags().description() {
            s.wln("##### Description");
            s.newline();
            for l in desc.as_doc_lines() {
                s.wln(l);
                s.newline();
            }
        }

        if let Some(comment) = t.tags().comment() {
            s.wln("##### Comment");
            s.newline();
            for l in comment.as_doc_lines() {
                s.wln(l);
                s.newline();
            }
        }

        s.wln("```c");
        let mut bytes = t.raw_bytes().iter();

        print_container_example_header(s, e, &mut bytes);

        let mut values = HashMap::new();

        for m in e.fields() {
            print_container_example_member(s, e, m, &mut bytes, &mut values, o, e.tags(), "");
        }

        s.wln("```");
    }
}

fn print_container_if_statement(
    s: &mut DocWriter,
    statement: &IfStatement,
    offset: &mut Option<usize>,
    tags: &Tags,
    o: &Objects,
) {
    s.w(format!("If {variable} ", variable = statement.name()));
    for (i, e) in statement.get_conditional().equations().iter().enumerate() {
        if i != 0 {
            s.wln(" **or** ");
        }
        s.w(&(match e {
            Equation::Equals { value } => format!("is equal to `{}`", value),
            Equation::NotEquals { value } => format!("is not equal to `{}`", value),
            Equation::BitwiseAnd { value } => format!("contains `{}`", value),
        }))
    }
    s.wln(":");
    s.newline();

    print_container_item_header(s);

    for m in statement.members() {
        print_container_field(s, m, offset, tags, o);
    }

    if !statement.else_ifs().is_empty() {
        for elseif in statement.else_ifs() {
            s.newline();
            s.w("Else ");
            print_container_if_statement(s, elseif, offset, tags, o);
        }
    }

    if !statement.else_members().is_empty() {
        s.newline();
        s.wln("Else: ");

        for m in statement.else_members() {
            print_container_field(s, m, offset, tags, o);
        }
    }
}

fn print_container_field(
    s: &mut DocWriter,
    m: &StructMember,
    offset: &mut Option<usize>,
    tags: &Tags,
    o: &Objects,
) {
    match m {
        StructMember::Definition(d) => {
            let ty = match d.ty() {
                Type::Identifier { s, .. } => {
                    format!("[{}]({}.md)", d.ty().str(), s.to_lowercase())
                }
                Type::PackedGuid | Type::Guid => {
                    format!("[{}](../spec/packed-guid.md)", d.ty().str())
                }
                Type::UpdateMask => {
                    format!("[{}](../spec/update-mask.md)", d.ty().str())
                }
                Type::AuraMask => {
                    format!("[{}](../spec/aura-mask.md)", d.ty().str())
                }
                Type::Array(array) => match array.ty() {
                    ArrayType::CString | ArrayType::Integer(_) => d.ty().str(),
                    ArrayType::Complex(identifier) => {
                        format!(
                            "[{ty}]({ty_path}.md)[{size}]",
                            ty = identifier,
                            ty_path = identifier.to_lowercase(),
                            size = array.size().str(),
                        )
                    }
                    ArrayType::Guid => {
                        format!(
                            "[Guid](../spec/packed-guid.md)[{size}]",
                            size = array.size().str()
                        )
                    }
                    ArrayType::PackedGuid => {
                        format!(
                            "[PackedGuid](../spec/packed-guid.md)[{size}]",
                            size = array.size().str()
                        )
                    }
                },
                Type::SizedCString
                | Type::Bool
                | Type::DateTime
                | Type::CString
                | Type::String { .. }
                | Type::Integer(_)
                | Type::FloatingPoint(_) => d.ty().str(),
            };

            let description = if let Some(d) = d.tags().description() {
                d.as_doc_table_string()
            } else {
                "".to_string()
            };
            let comment = if let Some(d) = d.tags().comment() {
                d.as_doc_table_string()
            } else {
                "".to_string()
            };

            s.wln(format!(
                "| {offset} | {size} / {endian} | {ty} | {name} | {description} | {comment} |",
                offset = if let Some(offset) = offset {
                    format!("0x{:02X}", offset)
                } else {
                    "-".to_string()
                },
                size = d.ty().doc_size_of(tags, o),
                endian = d.ty().doc_endian_str(),
                ty = ty,
                name = d.name(),
                description = description,
                comment = comment,
            ));

            if offset.is_some() {
                *offset = match d.ty() {
                    Type::Integer(t) => Some(offset.unwrap() + t.size() as usize),
                    Type::Guid => Some(offset.unwrap() + 8),
                    Type::FloatingPoint(f) => Some(offset.unwrap() + f.size() as usize),
                    Type::DateTime => Some(offset.unwrap() + DATETIME_SIZE as usize),
                    Type::Bool => Some(offset.unwrap() + BOOL_SIZE as usize),
                    Type::Identifier { s, upcast } => {
                        if let Some(upcast) = upcast {
                            Some(offset.unwrap() + upcast.size() as usize)
                        } else {
                            let sizes = o.get_object(s, tags).sizes();
                            sizes.is_constant().map(|size| offset.unwrap() + size)
                        }
                    }
                    Type::CString
                    | Type::SizedCString
                    | Type::String { .. }
                    | Type::Array(_)
                    | Type::PackedGuid
                    | Type::UpdateMask
                    | Type::AuraMask => None,
                };
            }
        }
        StructMember::IfStatement(statement) => {
            s.newline();

            print_container_if_statement(s, statement, offset, tags, o);
        }
        StructMember::OptionalStatement(_) => {}
    }
}

fn print_container_item_header(s: &mut DocWriter) {
    s.wln("| Offset | Size / Endianness | Type | Name | Description | Comment |");
    s.wln("| ------ | ----------------- | ---- | ---- | ----------- | ------- |");
}

fn print_container_body(s: &mut DocWriter, e: &Container, o: &Objects) {
    s.wln("### Body");
    s.newline();

    if e.tags().unimplemented() {
        s.wln("The body for this message has not been implemented yet.");
        s.newline();
        return;
    }

    if e.fields().is_empty() {
        s.wln("This message has no fields in the body.");
        s.newline();
        return;
    }

    let mut offset = Some(match e.container_type() {
        ContainerType::Msg(_) => 0,
        ContainerType::CMsg(_) => 6,
        ContainerType::SMsg(_) => 4,
        ContainerType::CLogin(_) | ContainerType::SLogin(_) => 1,
        ContainerType::Struct => 0,
    });

    if !(e.fields().len() == 1 && matches!(&e.fields()[0], &StructMember::OptionalStatement(_))) {
        print_container_item_header(s);
    }

    for m in e.fields() {
        print_container_field(s, m, &mut offset, e.tags(), o);
    }

    if e.rust_object().optional().is_some() {
        s.newline();
        s.wln("Optionally the following fields can be present. This can only be detected by looking at the size of the message.");
        s.newline();

        print_container_item_header(s);

        for m in e.fields() {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(_) => {}
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        print_container_field(s, m, &mut offset, e.tags(), o);
                    }
                }
            }
        }
    }

    s.newline();
}

fn print_container_header(s: &mut DocWriter, e: &Container) {
    if e.container_type() == ContainerType::Struct {
        return;
    }

    s.wln("### Header");
    s.newline();

    match e.container_type() {
        ContainerType::Msg(_) => s.wln("MSG have a header of either 6 bytes if they are sent from the client (CMSG), or 4 bytes if they are sent from the server (SMSG)."),
        ContainerType::CMsg(_) => {
            s.wln("CMSG have a header of 6 bytes.");
        }
        ContainerType::SMsg(_) => {
            s.wln("SMSG have a header of 4 bytes.");
        }
        ContainerType::SLogin(_) | ContainerType::CLogin(_) => {
            s.wln("Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.")
        }
        _ => panic!("unexpected container type"),
    };

    s.newline();

    if matches!(
        e.container_type(),
        ContainerType::CMsg(_) | ContainerType::Msg(_)
    ) {
        s.wln("#### CMSG Header");
        s.newline();

        s.wln("| Offset | Size / Endianness | Type   | Name   | Description |");
        s.wln("| ------ | ----------------- | ------ | ------ | ----------- |");
        s.wln("| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|");
        s.wln("| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|");
    }

    if matches!(
        e.container_type(),
        ContainerType::SMsg(_) | ContainerType::Msg(_)
    ) {
        s.wln("#### SMSG Header");
        s.newline();

        s.wln("| Offset | Size / Endianness | Type   | Name   | Description |");
        s.wln("| ------ | ----------------- | ------ | ------ | ----------- |");
        s.wln("| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|");
        s.wln("| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|");
    }

    if matches!(
        e.container_type(),
        ContainerType::SLogin(_) | ContainerType::CLogin(_)
    ) {
        s.wln("#### Login Header");
        s.newline();

        s.wln("| Offset | Size / Endianness | Type   | Name   | Description |");
        s.wln("| ------ | ----------------- | ------ | ------ | ----------- |");
        s.wln("| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|");
    }

    s.newline();
}

fn get_integer_value(t: &IntegerType, value: &[u8]) -> isize {
    match t {
        IntegerType::U8 => value[0] as isize,
        IntegerType::U16(e) => {
            let value: [u8; 2] = value.try_into().unwrap();
            match e {
                Endianness::Little => u16::from_le_bytes(value) as isize,
                Endianness::Big => u16::from_be_bytes(value) as isize,
            }
        }
        IntegerType::U32(e) => {
            let value: [u8; 4] = value.try_into().unwrap();
            match e {
                Endianness::Little => u32::from_le_bytes(value) as isize,
                Endianness::Big => u32::from_be_bytes(value) as isize,
            }
        }
        IntegerType::U64(e) => {
            let value: [u8; 8] = value.try_into().unwrap();
            match e {
                Endianness::Little => u64::from_le_bytes(value) as isize,
                Endianness::Big => u64::from_be_bytes(value) as isize,
            }
        }
        IntegerType::I32(e) => {
            let value: [u8; 4] = value.try_into().unwrap();
            match e {
                Endianness::Little => i32::from_le_bytes(value) as isize,
                Endianness::Big => i32::from_be_bytes(value) as isize,
            }
        }
    }
}
