use crate::file_utils::get_import_path;
use crate::parser::enumerator::Definer;
use crate::rust_printer::{print_docc_description_and_comment, Version, Writer};
use crate::wowm_printer::get_definer_wowm_definition;
use crate::{Objects, DISPLAY_STR};

pub fn print_common_enum_common(e: &Definer, o: &Objects, version: Version) -> Writer {
    print_enum_inner(e, o, version, true)
}

pub fn print_enum(e: &Definer, o: &Objects, version: Version) -> Writer {
    print_enum_inner(e, o, version, false)
}

fn print_enum_inner(
    e: &Definer,
    o: &Objects,
    version: Version,
    visibility_override: bool,
) -> Writer {
    let mut s = Writer::new(&get_import_path(version));

    includes(&mut s);

    declaration(&mut s, e, o, visibility_override);

    common_impls(&mut s, e, visibility_override);

    print_default(&mut s, e);

    print_display(&mut s, e);

    print_from_or_try_from(&mut s, e);

    s
}

fn includes(s: &mut Writer) {
    s.wln("use std::convert::{TryFrom, TryInto};");

    s.newline();
}

fn declaration(s: &mut Writer, e: &Definer, o: &Objects, common_visibility_override: bool) {
    print_docc_description_and_comment(s, e.tags(), o, e.tags());
    print_wowm_definition("enum", s, e);

    s.wln("#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]");
    let visibility = match e.only_used_in_if() && !common_visibility_override {
        true => "pub(crate)",
        false => "pub",
    };
    s.new_enum(visibility, e.name(), |s| {
        for field in e.fields() {
            print_docc_description_and_comment(s, field.tags(), o, e.tags());

            s.wln(format!("{},", field.rust_name()));
        }

        match e.self_value() {
            None => {}
            Some(self_value) => {
                s.wln(format!(
                    "{}({}),",
                    self_value.rust_name(),
                    e.ty().rust_str()
                ));
            }
        }
    });
}

pub fn print_wowm_definition(kind: &str, s: &mut Writer, e: &Definer) {
    s.docc_wowm(
        |s| {
            s.wln(get_definer_wowm_definition(kind, e, "/// "));
        },
        e.file_info(),
    );
}

fn common_impls(s: &mut Writer, e: &Definer, visibility_override: bool) {
    s.bodyn(format!("impl {}", e.name()), |s| {
        as_type(s, e, visibility_override);
    });
}

fn as_type(s: &mut Writer, e: &Definer, visibility_override: bool) {
    let f = if visibility_override {
        Writer::funcn_pub_const
    } else {
        Writer::funcn_const
    };

    f(s, "as_int(&self)", e.ty().rust_str(), |s: &mut Writer| {
        s.body("match self", |s| {
            for field in e.fields() {
                s.wln(format!(
                    "Self::{} => 0x{:x},",
                    field.rust_name(),
                    field.value().int()
                ));
            }
            match e.self_value() {
                None => {}
                Some(self_value) => {
                    s.wln(format!("Self::{}(v) => *v,", self_value.rust_name()));
                }
            }
        });
    });
}

fn print_default(s: &mut Writer, e: &Definer) {
    s.impl_for("Default", e.name(), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln(format!("Self::{}", e.fields()[0].rust_name()));
        });
    });
}

fn print_display(s: &mut Writer, e: &Definer) {
    s.impl_for("std::fmt::Display", e.name(), |s| {
        s.body(
            "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
            |s| {
                s.body("match self", |s| {
                    for field in e.fields() {
                        let display = match field.tags().get_ref(DISPLAY_STR) {
                            None => field.rust_name(),
                            Some(v) => v,
                        };
                        s.wln(format!(
                            r#"Self::{name} => f.write_str("{display}"),"#,
                            name = field.rust_name(),
                            display = display,
                        ));
                    }

                    match e.self_value() {
                        None => {}
                        Some(self_value) => {
                            s.wln(format!(
                                r#"Self::{name}(v) => f.write_fmt(format_args!("{name}({{}})", v)),"#,
                                name = self_value.rust_name(),
                            ));
                        }
                    }
                });
            },
        );
    });
}

fn print_from_or_try_from(s: &mut Writer, e: &Definer) {
    if e.self_value().is_none() {
        print_try_from(s, e);
    } else {
        print_from(s, e);
    }
}

fn print_try_from(s: &mut Writer, e: &Definer) {
    s.impl_for(format!("TryFrom<{}>", e.ty().rust_str()), e.name(), |s| {
        s.wln("type Error = crate::errors::EnumError;");

        s.body(
            format!(
                "fn try_from(value: {}) -> std::result::Result<Self, Self::Error>",
                e.ty().rust_str()
            ),
            |s| {
                s.body("match value", |s| {
                    for field in e.fields() {
                        s.wln(format!(
                            "{} => Ok(Self::{}),",
                            field.value().int(),
                            field.rust_name()
                        ));
                    }

                    s.wln(format!(
                        "v => Err(crate::errors::EnumError::new(\"{}\", v as u32),)",
                        e.name()
                    ));
                });
            },
        );
    });
}

fn print_from(s: &mut Writer, e: &Definer) {
    s.impl_for(format!("From<{}>", e.ty().rust_str()), e.name(), |s| {
        s.body(
            format!("fn from(value: {}) -> Self", e.ty().rust_str()),
            |s| {
                s.body("match value", |s| {
                    for field in e.fields() {
                        s.wln(format!(
                            "{} => Self::{},",
                            field.value().int(),
                            field.rust_name()
                        ));
                    }

                    s.wln(format!(
                        "v => Self::{}(v)",
                        e.self_value().as_ref().unwrap().rust_name()
                    ));
                });
            },
        );
    });
}
