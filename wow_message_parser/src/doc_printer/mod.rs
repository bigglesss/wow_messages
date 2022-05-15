use crate::container::{Container, Equation, IfStatement, StructMember, StructMemberDefinition};
use crate::file_info::FileInfo;
use crate::file_utils::{create_or_append, write_string_to_file};
use crate::parser::enumerator::Definer;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::ty::Type;
use crate::parser::types::{Array, ArraySize, ArrayType, Endianness, IntegerType, ObjectType};
use crate::wowm_printer::{get_definer_wowm_definition, get_struct_wowm_definition};
use crate::ContainerType;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;
use std::slice::Iter;

pub struct DocWriter {
    name: String,
    inner: String,
    column: usize,
    tags: Tags,
}

impl DocWriter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }

    pub fn new(name: &str, tags: &Tags) -> Self {
        Self {
            name: name.to_string(),
            inner: String::with_capacity(8000),
            column: 0,
            tags: tags.clone(),
        }
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.column += s.as_ref().len();
    }

    pub fn w_break_at(&mut self, s: impl AsRef<str>) {
        self.w(s);
        if self.column > 80 {
            self.newline();
        }
    }

    pub fn newline(&mut self) {
        self.w("\n");
        self.column = 0;
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn bytes<'a>(&mut self, bytes: impl Iterator<Item = &'a u8>) {
        for b in bytes {
            let text = format!("{}, ", b);
            self.w(&text);
            self.column += text.len();
        }
    }
}

pub fn print_docs_summary_and_objects(definers: &[DocWriter], containers: &[DocWriter]) {
    const LOGIN_DEFINER_HEADER: &str = "# Login Definers\n";
    const LOGIN_CONTAINER_HEADER: &str = "# Login Containers\n";
    const WORLD_DEFINER_HEADER: &str = "# World Definers\n";
    const WORLD_CONTAINER_HEADER: &str = "# Login Containers\n";
    const SUMMARY_PATH: &str = "wowm_language/src/SUMMARY.md";

    let s = read_to_string(SUMMARY_PATH).unwrap();
    let (s, _) = s.split_once(LOGIN_DEFINER_HEADER).unwrap();
    let mut s = s.to_string();

    let mut already_added_files = Vec::new();
    let mut login_definers = Vec::new();
    let mut world_definers = Vec::new();

    for definer in definers {
        let path = format!(
            "docs/{lower_name}.md",
            lower_name = definer.name().to_lowercase()
        );

        create_or_append(
            definer.inner(),
            Path::new(&("wowm_language/src/".to_string() + &path)),
        );

        if already_added_files.contains(&path) {
            continue;
        }

        let bullet_point = format!("- [{name}]({path})\n", name = definer.name(), path = path,);
        if definer.tags().has_login_version() {
            login_definers.push(bullet_point)
        } else {
            world_definers.push(bullet_point);
        }

        already_added_files.push(path);
    }

    s.push_str(LOGIN_DEFINER_HEADER);
    for i in login_definers {
        s.push_str(&i);
    }
    s.push('\n');

    s.push_str(WORLD_DEFINER_HEADER);
    for i in world_definers {
        s.push_str(&i);
    }
    s.push('\n');

    let mut login_containers = Vec::new();
    let mut world_containers = Vec::new();
    for container in containers {
        let path = format!(
            "docs/{lower_name}.md",
            lower_name = container.name().to_lowercase()
        );

        create_or_append(
            container.inner(),
            Path::new(&("wowm_language/src/".to_string() + &path)),
        );

        if already_added_files.contains(&path) {
            continue;
        }

        let bullet_point = format!("- [{name}]({path})\n", name = container.name(), path = path,);
        if container.tags().has_login_version() {
            login_containers.push(bullet_point)
        } else {
            world_containers.push(bullet_point);
        }

        already_added_files.push(path);
    }

    s.push_str(LOGIN_CONTAINER_HEADER);
    for i in login_containers {
        s.push_str(&i);
    }
    s.push('\n');

    s.push_str(WORLD_CONTAINER_HEADER);
    for i in world_containers {
        s.push_str(&i);
    }
    s.push('\n');

    write_string_to_file(&s, Path::new(SUMMARY_PATH))
}

fn common(s: &mut DocWriter, tags: &Tags) {
    print_versions(s, tags.logon_versions(), tags.versions());

    print_metadata(s, tags);
}

fn definer_common(s: &mut DocWriter, tags: &Tags, fileinfo: &FileInfo, ty: &str, e: &Definer) {
    common(s, tags);

    s.wln("### Wowm Representation");
    s.newline();

    s.wln(format!(
        "Autogenerated from `wowm` file at {github_link}.",
        github_link = fileinfo.original_file_github_link()
    ));
    s.newline();
    s.wln("```rust,ignore");
    s.w(get_definer_wowm_definition(ty, e, ""));
    s.wln("```");
}

fn print_metadata(s: &mut DocWriter, tags: &Tags) {
    if let Some(description) = tags.description() {
        s.wln("### Description");
        s.wln(description);
        s.newline();
    }

    if let Some(comment) = tags.comment() {
        s.wln("### Comment");
        s.newline();
        s.wln(comment);
        s.newline();
    }
}

fn print_versions(
    s: &mut DocWriter,
    login_versions: &[LoginVersion],
    world_versions: &[WorldVersion],
) {
    s.w("## ");

    for (i, l) in login_versions.iter().enumerate() {
        s.w(format!("Protocol Version {}", l.to_string()));
        if i != login_versions.len() - 1 {
            s.w(format!(", "));
        }
    }

    for (i, l) in world_versions.iter().enumerate() {
        s.w(format!("Client Version {}", l.to_string()));
        if i != world_versions.len() - 1 {
            s.w(format!(", "));
        }
    }

    s.newline();
    s.newline();
}

pub fn print_docs_for_enum(e: &Definer) -> DocWriter {
    let mut s = DocWriter::new(e.name(), e.tags());

    definer_common(&mut s, e.tags(), e.file_info(), "enum", e);

    print_definer_table(&mut s, e);

    s.wln("Used in:");
    for c in e.objects_used_in() {
        s.wln(format!(
            "* [{ty}]({ty_path}.md)",
            ty = c.0,
            ty_path = c.0.to_lowercase(),
        ));
    }

    s
}

pub fn print_docs_for_flag(e: &Definer) -> DocWriter {
    let mut s = DocWriter::new(e.name(), e.tags());

    definer_common(&mut s, e.tags(), e.file_info(), "flag", e);

    print_definer_table(&mut s, e);

    s.wln("Used in:");
    for c in e.objects_used_in() {
        s.wln(format!(
            "* [{ty}]({ty_path}.md)",
            ty = c.0,
            ty_path = c.0.to_lowercase(),
        ));
    }

    s
}

pub fn print_docs_for_container(e: &Container, o: &Objects) -> DocWriter {
    let mut s = DocWriter::new(e.name(), e.tags());

    common(&mut s, e.tags());

    s.wln("### Wowm Representation");
    s.wln("```rust,ignore");
    s.w(get_struct_wowm_definition(e, ""));
    s.wln("```");

    print_container_header(&mut s, e);
    print_container_body(&mut s, e);

    print_container_examples(&mut s, e, o);

    s
}

fn get_integer_value(t: &IntegerType, value: &[u8]) -> usize {
    match t {
        IntegerType::U8 => value[0] as usize,
        IntegerType::U16(e) => {
            let value: [u8; 2] = value.try_into().unwrap();
            match e {
                Endianness::Little => u16::from_le_bytes(value) as usize,
                Endianness::Big => u16::from_be_bytes(value) as usize,
            }
        }
        IntegerType::U32(e) => {
            let value: [u8; 4] = value.try_into().unwrap();
            match e {
                Endianness::Little => u32::from_le_bytes(value) as usize,
                Endianness::Big => u32::from_be_bytes(value) as usize,
            }
        }
        IntegerType::U64(e) => {
            let value: [u8; 8] = value.try_into().unwrap();
            match e {
                Endianness::Little => u64::from_le_bytes(value) as usize,
                Endianness::Big => u64::from_be_bytes(value) as usize,
            }
        }
    }
}

fn print_container_example_array(
    s: &mut DocWriter,
    array: &Array,
    bytes: &mut Iter<u8>,
    values: &mut HashMap<String, usize>,
    o: &Objects,
    tags: &Tags,
    prefix: &str,
) {
    let size = match array.size() {
        ArraySize::Fixed(size) => size as usize,
        ArraySize::Variable(length) => *values.get(&length).unwrap(),
        ArraySize::Endless => {
            let size = bytes.size_hint();
            assert_eq!(size.0, size.1.unwrap());
            size.0
        }
    };

    for _ in 0..size {
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

                for (i, m) in c.fields().iter().enumerate() {
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
    values: &mut HashMap<String, usize>,
    o: &Objects,
    tags: &Tags,
    prefix: &str,
) {
    let comment = if prefix != "" {
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
        Type::Guid => {
            s.bytes(bytes.take(core::mem::size_of::<u64>()).into_iter());
        }
        Type::FloatingPoint(f) => {
            s.bytes(bytes.take(f.size() as usize).into_iter());
        }
        Type::PackedGuid => {
            let mask = bytes.next().unwrap();
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
                *values.get(length).unwrap()
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
                        let mut t = comment.to_string();
                        t.push(' ');

                        for (i, v) in values.iter().enumerate() {
                            if i != 0 {
                                t.push_str("| ");
                            } else {
                                t.push_str(" ");
                            }
                            t.push_str(v.name());
                        }

                        t.push_str(&format!(" ({})", value));
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
        Type::UpdateMask => panic!("UpdateMask example"),
        Type::AuraMask => panic!("AuraMask example"),
    }
    s.wln(comment);
}

fn print_container_example_member(
    s: &mut DocWriter,
    e: &Container,
    m: &StructMember,
    bytes: &mut Iter<u8>,
    values: &mut HashMap<String, usize>,
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

            let statement_set = |statement: &IfStatement, enum_value: usize| {
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
            panic!("UNIMPLEMENTED_DOC_OPTIONAL");
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

    for t in e.tests() {
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
) {
    s.w(format!("If {variable} ", variable = statement.name()));
    for (i, e) in statement.get_conditional().equations().iter().enumerate() {
        if i != 0 {
            s.wln(" **or** ");
        }
        s.w(format!(
            "{}",
            match e {
                Equation::Equals { value } => format!("is equal to `{}`", value),
                Equation::NotEquals { value } => format!("is not equal to `{}`", value),
                Equation::BitwiseAnd { value } => format!("contains `{}`", value),
            }
        ))
    }
    s.wln(":");
    s.newline();

    print_container_item_header(s);

    for m in statement.members() {
        print_container_field(s, m, offset);
    }

    if !statement.else_ifs().is_empty() {
        for elseif in statement.else_ifs() {
            s.newline();
            s.w(format!("Else "));
            print_container_if_statement(s, elseif, offset);
        }
    }

    if !statement.else_members().is_empty() {
        s.newline();
        s.wln("Else: ");

        for m in statement.else_members() {
            print_container_field(s, m, offset);
        }
    }
}

fn print_container_field(s: &mut DocWriter, m: &StructMember, offset: &mut Option<usize>) {
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
                Type::CString | Type::String { .. } | Type::Integer(_) | Type::FloatingPoint(_) => {
                    d.ty().str()
                }
            };
            s.wln(format!(
                "| {offset} | {size} / {endian} | {ty} | {name} | {description} |",
                offset = if let Some(offset) = offset {
                    format!("0x{:02X}", offset)
                } else {
                    "-".to_string()
                },
                size = d.ty().doc_size_of(),
                endian = d.ty().doc_endian_str(),
                ty = ty,
                name = d.name(),
                description = d.tags().description().unwrap_or(""),
            ));

            if let Some(_) = offset {
                *offset = match d.ty() {
                    Type::Integer(t) => Some(offset.unwrap() + t.size() as usize),
                    Type::Guid => Some(offset.unwrap() + 8),
                    Type::FloatingPoint(f) => Some(offset.unwrap() + f.size() as usize),
                    Type::CString
                    | Type::String { .. }
                    | Type::Identifier { .. }
                    | Type::Array(_)
                    | Type::PackedGuid
                    | Type::UpdateMask
                    | Type::AuraMask => None,
                };
            }
        }
        StructMember::IfStatement(statement) => {
            s.newline();

            print_container_if_statement(s, statement, offset);
        }
        StructMember::OptionalStatement(_) => {}
    }
}

fn print_container_item_header(s: &mut DocWriter) {
    s.wln("| Offset | Size / Endianness | Type | Name | Description |");
    s.wln("| ------ | ----------------- | ---- | ---- | ----------- |");
}

fn print_container_body(s: &mut DocWriter, e: &Container) {
    s.wln("### Body");

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
        print_container_field(s, m, &mut offset);
    }

    if let Some(_) = e.rust_object().optional() {
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
                        print_container_field(s, m, &mut offset);
                    }
                }
            }
        }
    }
}

fn print_container_header(s: &mut DocWriter, e: &Container) {
    match e.container_type() {
        ContainerType::Struct => return,
        _ => {}
    }

    s.wln("### Header");

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

        s.wln("| Offset | Size / Endianness | Type   | Name   | Description |");
        s.wln("| ------ | ----------------- | ------ | ------ | ----------- |");
        s.wln("| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|");
    }
}

fn print_definer_table(s: &mut DocWriter, e: &Definer) {
    s.wln("### Type");
    s.wln(format!(
        "The basic type is `{ty_str}`, a {byte} byte ({bit} bit){endian} integer.",
        ty_str = e.ty().str(),
        byte = e.ty().size(),
        bit = e.ty().size() * 8,
        endian = match e.ty() {
            IntegerType::U8 => "".to_string(),
            IntegerType::U16(e) | IntegerType::U32(e) | IntegerType::U64(e) => format!(
                " {} endian",
                match e {
                    Endianness::Little => "little",
                    Endianness::Big => "big",
                }
            ),
        }
    ));

    s.wln("### Enumerators");

    let any_fields_has_display = e.fields().iter().any(|f| f.tags().display().is_some());

    s.w("| Enumerator | Value  | Description | Comment |");
    if any_fields_has_display {
        s.wln(" Display |");
    } else {
        s.newline();
    }

    s.w("| --------- | -------- | ----------- | ------- |");
    if any_fields_has_display {
        s.wln(" ------- |");
    } else {
        s.newline();
    }

    for f in e.fields() {
        s.w(format!(
            "| `{name}` | {value} (0x{hex:0>2X}) | {description} | {comment} |",
            name = f.name(),
            value = f.value().int(),
            hex = f.value().int(),
            description = f.tags().description().unwrap_or(""),
            comment = f.tags().comment().unwrap_or(""),
        ));
        if any_fields_has_display {
            s.wln(format!(" {} |", f.tags().display().unwrap_or("")));
        } else {
            s.newline();
        }
    }

    s.newline();
}
