use crate::doc_printer::DocWriter;
use crate::file_info::FileInfo;
use crate::parser::enumerator::Definer;
use crate::parser::types::{Endianness, IntegerType};
use crate::wowm_printer::get_definer_wowm_definition;
use crate::{doc_printer, Tags};

fn definer_common(s: &mut DocWriter, tags: &Tags, fileinfo: &FileInfo, ty: &str, e: &Definer) {
    doc_printer::common(s, tags);

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

fn print_definer_table(s: &mut DocWriter, e: &Definer) {
    s.wln("### Type");
    s.wln(format!(
        "The basic type is `{ty_str}`, a {byte} byte ({bit} bit){endian} integer.",
        ty_str = e.ty().str(),
        byte = e.ty().size(),
        bit = e.ty().size() * 8,
        endian = match e.ty() {
            IntegerType::U8 => "".to_string(),
            IntegerType::U16(e)
            | IntegerType::U32(e)
            | IntegerType::U64(e)
            | IntegerType::I32(e) => format!(
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
        let description = if let Some(d) = f.tags().description() {
            d.as_doc_table_string()
        } else {
            "".to_string()
        };

        let comment = if let Some(d) = f.tags().comment() {
            d.as_doc_table_string()
        } else {
            "".to_string()
        };

        s.w(format!(
            "| `{name}` | {value} (0x{hex:0>2X}) | {description} | {comment} |",
            name = f.name(),
            value = f.value().int(),
            hex = f.value().int(),
            description = description,
            comment = comment,
        ));
        if any_fields_has_display {
            s.wln(format!(" {} |", f.tags().display().unwrap_or("")));
        } else {
            s.newline();
        }
    }

    s.newline();
}
