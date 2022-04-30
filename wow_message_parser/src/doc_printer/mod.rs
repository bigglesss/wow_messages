use crate::container::{Container, Equation, IfStatement, StructMember};
use crate::file_utils::{append_string_to_file, create_or_append, write_string_to_file};
use crate::parser::enumerator::Definer;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::ty::Type;
use crate::parser::types::{Endianness, IntegerType};
use crate::wowm_printer::{get_definer_wowm_definition, get_struct_wowm_definition};
use crate::ContainerType;
use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;

pub struct DocWriter {
    name: String,
    inner: String,
}

impl DocWriter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inner: String::with_capacity(8000),
        }
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.w("\n");
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }
}

pub fn print_docs_summary_and_objects(definers: &[DocWriter], containers: &[DocWriter]) {
    const DEFINER_HEADER: &str = "# Autogenerated Docs (Definers)\n";
    const CONTAINER_HEADER: &str = "# Autogenerated Docs (Containers)\n";
    const SUMMARY_PATH: &str = "wowm_language/src/SUMMARY.md";

    let s = read_to_string(SUMMARY_PATH).unwrap();
    let (s, _) = s.split_once(DEFINER_HEADER).unwrap();
    let mut s = s.to_string();
    s.push_str(DEFINER_HEADER);

    let mut already_added_files = Vec::new();

    for definer in definers {
        let path = format!(
            "docs/definer/{lower_name}.md",
            lower_name = definer.name().to_lowercase()
        );

        create_or_append(
            definer.inner(),
            Path::new(&("wowm_language/src/".to_string() + &path)),
        );

        if already_added_files.contains(&path) {
            continue;
        }

        s.write_fmt(format_args!(
            "- [{name}]({path})\n",
            name = definer.name(),
            path = path,
        ))
        .unwrap();

        already_added_files.push(path);
    }

    s.push('\n');
    s.push_str(CONTAINER_HEADER);
    for container in containers {
        let path = format!(
            "docs/container/{lower_name}.md",
            lower_name = container.name().to_lowercase()
        );

        create_or_append(
            container.inner(),
            Path::new(&("wowm_language/src/".to_string() + &path)),
        );

        if already_added_files.contains(&path) {
            continue;
        }

        s.write_fmt(format_args!(
            "- [{name}]({path})\n",
            name = container.name(),
            path = path,
        ))
        .unwrap();

        already_added_files.push(path);
    }

    write_string_to_file(&s, Path::new(SUMMARY_PATH))
}

fn common(s: &mut DocWriter, tags: &Tags) {
    print_versions(s, tags.logon_versions(), tags.versions());

    print_metadata(s, tags);
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
    let mut s = DocWriter::new(e.name());

    common(&mut s, e.tags());

    s.wln("### Wowm Representation");
    s.wln("```rust,ignore");
    s.wln(get_definer_wowm_definition("enum", e, ""));
    s.wln("```");

    print_definer_table(&mut s, e);

    s
}

pub fn print_docs_for_flag(e: &Definer) -> DocWriter {
    let mut s = DocWriter::new(e.name());

    common(&mut s, e.tags());

    s.wln("### Wowm Representation");
    s.wln("```rust,ignore");
    s.wln(get_definer_wowm_definition("flag", e, ""));
    s.wln("```");

    print_definer_table(&mut s, e);

    s
}

pub fn print_docs_for_container(e: &Container) -> DocWriter {
    let mut s = DocWriter::new(e.name());

    common(&mut s, e.tags());

    s.wln("### Wowm Representation");
    s.wln("```rust,ignore");
    s.wln(get_struct_wowm_definition(e, ""));
    s.wln("```");

    print_container_header(&mut s, e);
    print_container_body(&mut s, e);

    s
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

    if !statement.else_statement_members.is_empty() {
        s.newline();
        s.wln("Else: ");

        for m in &statement.else_statement_members {
            print_container_field(s, m, offset);
        }
    }
}

fn print_container_field(s: &mut DocWriter, m: &StructMember, offset: &mut Option<usize>) {
    match m {
        StructMember::Definition(d) => {
            s.wln(format!(
                "| {offset} | {size} / {endian} | {ty} | {name} | {description} |",
                offset = if let Some(offset) = offset {
                    format!("0x{:02X}", offset)
                } else {
                    "-".to_string()
                },
                size = d.ty().doc_size_of(),
                endian = d.ty().doc_endian_str(),
                ty = d.ty().str(),
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
        ContainerType::Struct | ContainerType::CLogin(_) | ContainerType::SLogin(_) => 0,
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
        ContainerType::Struct | ContainerType::CLogin(_) | ContainerType::SLogin(_) => return,
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

    s.w("| Enumerator | Original  | Description | Comment |");
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
}
