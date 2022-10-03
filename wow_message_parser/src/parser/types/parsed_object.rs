use crate::parser::types::container::{Container, DefinerUsage};
use crate::parser::types::parsed_definer::ParsedDefiner;
use crate::parser::types::test_case::TestCase;
use crate::Objects;

#[derive(Debug, Clone)]
pub struct ParsedObjects {
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<Container>,
    messages: Vec<Container>,
    tests: Vec<TestCase>,
}

impl ParsedObjects {
    pub fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
        }
    }

    pub fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<Container>,
        messages: Vec<Container>,
        tests: Vec<TestCase>,
    ) -> Self {
        Self {
            enums,
            flags,
            structs,
            messages,
            tests,
        }
    }

    pub fn add_vecs(&mut self, mut c: Self) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
    }

    pub fn to_objects(self) -> Objects {
        Objects::new(
            self.enums,
            self.flags,
            self.structs,
            self.messages,
            self.tests,
        )
    }
}

pub fn get_definer_objects_used_in(
    messages: &[Container],
    structs: &[Container],
    e: &ParsedDefiner,
) -> Vec<(String, DefinerUsage)> {
    let mut v = Vec::new();

    for c in [messages, structs].concat() {
        if !e.tags().has_version_intersections(c.tags()) {
            continue;
        }

        let ty = match c.contains_definer(e.name()) {
            DefinerUsage::Unused => continue,
            DefinerUsage::NotInIf => DefinerUsage::NotInIf,
            DefinerUsage::InIf => DefinerUsage::InIf,
        };

        v.push((c.name().to_string(), ty));
    }

    v.sort_by(|a, b| a.0.cmp(&b.0));

    v
}
