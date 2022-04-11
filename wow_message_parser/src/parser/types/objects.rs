use crate::container::{Container, StructMember};
use crate::file_info::FileInfo;
use crate::parser::enumerator::Definer;
use crate::parser::stats::stats_for_1_12;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::{ArraySize, ArrayType, ObjectType, Type};
use crate::test_case::TestCase;

#[derive(Debug, Clone)]
pub struct Objects {
    enums: Vec<Definer>,
    flags: Vec<Definer>,
    structs: Vec<Container>,
    messages: Vec<Container>,
    tests: Vec<TestCase>,
}

impl Objects {
    pub fn new(
        enums: Vec<Definer>,
        flags: Vec<Definer>,
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

    pub fn get_definer(&self, ty_name: &str, tags: &Tags) -> &Definer {
        if let Some(d) = self
            .enums
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return d;
        }

        if let Some(d) = self
            .flags
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return d;
        }

        panic!("unable to find definer");
    }

    pub fn object_has_only_io_errors(&self, variable_name: &str, finder_tags: &Tags) -> bool {
        match self.get_object_type_of(variable_name, finder_tags) {
            ObjectType::Struct | ObjectType::CLogin | ObjectType::SLogin => {
                let c = self.get_container(variable_name, finder_tags);
                c.recursive_only_has_io_errors(self)
            }
            ObjectType::Enum => self
                .get_definer(variable_name, finder_tags)
                .self_value()
                .is_some(),
            ObjectType::Flag => true,
        }
    }

    pub fn get_object_type_of(&self, variable_name: &str, finder_tags: &Tags) -> ObjectType {
        if self
            .enums
            .iter()
            .any(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Enum;
        }

        if self
            .flags
            .iter()
            .any(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Flag;
        }

        if self
            .structs
            .iter()
            .any(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Struct;
        }

        panic!(
            "unable to find variable name: '{}' with tags: '{:#?}'",
            variable_name, finder_tags
        );
    }

    pub fn get_container(&self, name: &str, finder_tags: &Tags) -> &Container {
        if let Some(e) = self
            .all_containers()
            .find(|a| a.name() == name && a.tags().has_version_intersections(finder_tags))
        {
            return e;
        }

        panic!(
            "container not found: {} with tags: {:#?}",
            name, finder_tags
        );
    }

    pub fn get_tags_of_object(&self, type_name: &str, finder_tags: &Tags) -> &Tags {
        if let Some(e) = self
            .enums
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        if let Some(e) = self
            .flags
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        if let Some(e) = self
            .all_containers()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        panic!("unable to find type");
    }

    pub fn get_world_versions_with_objects(&self) -> Vec<WorldVersion> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            for l in s.tags().versions() {
                v.push(*l);
            }
        }

        v.sort();
        v.dedup();

        let index = v.iter().position(|a| a.eq(&WorldVersion::All));
        if let Some(index) = index {
            v.remove(index);
        }

        v
    }

    pub fn get_world_messages_with_versions_and_all(
        &self,
        version_number: &WorldVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            let logon = s.tags().versions();
            if logon.contains(version_number) || logon.contains(&WorldVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub fn get_login_versions_with_objects(&self) -> Vec<LoginVersion> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            for l in s.tags().logon_versions() {
                v.push(*l);
            }
        }

        v.sort();
        v.dedup();

        let index = v.iter().position(|a| a.eq(&LoginVersion::All));
        if let Some(index) = index {
            v.remove(index);
        }

        v
    }

    pub fn get_login_messages_with_versions_and_all(
        &self,
        version_number: &LoginVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            let logon = s.tags().logon_versions();
            if logon.contains(version_number) || logon.contains(&LoginVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub fn get_size_of_complex(&self, type_name: &str) -> u64 {
        if let Some(e) = self.all_definers().find(|a| a.name() == type_name) {
            return e.ty().size() as u64;
        }

        panic!(
            "complex types that are not enum/flag before size: '{type_name}'",
            type_name = type_name
        )
    }

    pub fn enums(&self) -> &[Definer] {
        &self.enums
    }

    pub fn flags(&self) -> &[Definer] {
        &self.flags
    }

    pub fn all_definers(&self) -> impl Iterator<Item = &Definer> {
        self.enums.iter().chain(&self.flags)
    }

    pub fn all_definers_mut(&mut self) -> impl Iterator<Item = &mut Definer> {
        self.enums.iter_mut().chain(&mut self.flags)
    }

    pub fn all_containers(&self) -> impl Iterator<Item = &Container> {
        self.structs.iter().chain(&self.messages)
    }

    pub fn all_containers_mut(&mut self) -> impl Iterator<Item = &mut Container> {
        self.structs.iter_mut().chain(&mut self.messages)
    }

    pub fn structs(&self) -> &[Container] {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut [Container] {
        &mut self.structs
    }

    pub fn messages(&self) -> &[Container] {
        &self.messages
    }

    pub fn messages_mut(&mut self) -> &mut [Container] {
        &mut self.messages
    }

    pub fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
        }
    }

    pub fn get_tests_for_object(
        tests: &mut Vec<TestCase>,
        name: &str,
        tags: &Tags,
    ) -> Vec<TestCase> {
        let mut v = Vec::new();
        let mut indices = Vec::new();

        for (i, t) in tests.iter().enumerate() {
            if t.subject() == name && t.tags().has_version_intersections(tags) {
                indices.push(i);
                v.push(t.clone());
            }
        }
        indices.reverse();

        for i in indices {
            tests.remove(i);
        }

        v
    }

    pub fn check_values(&mut self) {
        let c = self.clone();
        for s in &mut self.tests {
            s.verify(&c);
        }

        let mut tests = self.tests.clone();

        for s in self.all_containers_mut() {
            s.set_internals(&c);

            let t = Self::get_tests_for_object(&mut tests, s.name(), s.tags());
            s.append_tests(t);
        }

        Self::check_versions(self.all_containers(), self.all_definers());

        self.tests = tests;
    }

    fn check_versions<'a>(
        containers: impl Iterator<Item = &'a Container>,
        definers: impl Iterator<Item = &'a Definer>,
    ) {
        struct Obj<'a> {
            name: &'a str,
            tags: &'a Tags,
            file_info: &'a FileInfo,
        }

        let mut v: Vec<Obj> = Vec::new();
        for e in containers {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }
        for e in definers {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }

        for outer in &v {
            for inner in &v {
                if outer.name == inner.name
                    && outer.tags.has_version_intersections(inner.tags)
                    && outer.name as *const _ != inner.name as *const _
                {
                    panic!(
                        "Objects with same name and overlapping versions: {}
version 1: {:#?} in {} line {},
version 2: {:#?} in {} line {}",
                        inner.name,
                        inner.tags,
                        inner.file_info.name(),
                        inner.file_info.start_line(),
                        outer.tags,
                        outer.file_info.name(),
                        outer.file_info.start_line(),
                    );
                }
            }
        }
    }

    pub fn type_has_constant_size(&self, ty: &Type) -> bool {
        let type_name = match ty {
            Type::Integer(_) => return true,
            Type::FloatingPoint(_) => return true,
            Type::CString | Type::String { .. } => return false,
            Type::Array(array) => match array.size() {
                ArraySize::Fixed(_) => match array.ty() {
                    ArrayType::Integer(_) => return true,
                    ArrayType::Complex(ident) => ident,
                    ArrayType::CString => return false,
                    ArrayType::Guid => return true,
                    ArrayType::PackedGuid => return false,
                },
                ArraySize::Variable(_) => return false,
                ArraySize::Endless => return false,
            },
            Type::Identifier { s, .. } => s,
            Type::PackedGuid => return false,
            Type::Guid => return true,
            Type::UpdateMask => return false,
            Type::AuraMask => return false,
        };

        if self.all_definers().any(|a| a.name() == type_name) {
            return true;
        }

        if let Some(s) = self.all_containers().find(|a| a.name() == type_name) {
            return s.has_constant_size(self);
        }

        for s in self.all_containers() {
            if let Some(ce) = s
                .nested_types()
                .new_enums()
                .iter()
                .find(|&a| a.name() == type_name)
            {
                for f in ce.fields() {
                    for sf in f.subfields() {
                        match self.type_has_constant_size(sf.ty()) {
                            true => {}
                            false => return false,
                        }
                    }
                }

                return true;
            }
        }

        panic!(
            "Type name: '{type_name}' was not found.",
            type_name = type_name
        );
    }

    pub fn check_value(&self, i: &StructMember, tags: &Tags) {
        match i {
            StructMember::Definition(d) => match &d.ty() {
                Type::Integer(_) => {}
                Type::FloatingPoint(_) => {}
                Type::CString => {}
                Type::String { .. } => {}
                Type::Array(a) => match &a.inner {
                    ArrayType::Integer(_) => {}
                    ArrayType::Complex(c) => self.contains_complex_type(c, tags, d.name()),
                    ArrayType::CString => {}
                    ArrayType::Guid => {}
                    ArrayType::PackedGuid => {}
                },
                Type::Identifier { s: i, .. } => {
                    self.contains_complex_type(i, tags, d.name());
                    match d.value() {
                        None => {}
                        Some(v) => match v.identifier().parse::<usize>() {
                            Ok(_) => {}
                            Err(_) => {
                                self.contains_value_in_type(i, v.identifier());
                            }
                        },
                    }
                }
                Type::PackedGuid => {}
                Type::Guid => {}
                Type::UpdateMask => {}
                Type::AuraMask => {}
            },
            StructMember::IfStatement(statement) => {
                for member in &statement.members {
                    self.check_value(member, tags);
                }
                for member in &statement.else_statement_members {
                    self.check_value(member, tags);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    self.check_value(m, tags);
                }
            }
        }
    }

    fn contains_value_in_type(&self, variable_name: &str, value_name: &str) {
        let enums = self.all_definers().find(|a| a.name() == variable_name);
        match enums {
            None => {}
            Some(v) => {
                for a in v.fields() {
                    if a.name() == value_name {
                        return;
                    }
                }
            }
        }

        panic!(
            "value: '{}' not found in variable: '{}'",
            value_name, variable_name
        );
    }

    fn contains_complex_type(&self, variable_name: &str, tags: &Tags, struct_name: &str) {
        for e in self.all_definers() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
                return;
            }
        }

        for e in self.all_containers() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
                return;
            }
        }

        panic!(
            "Complex type not found: '{}' for object: '{}' for versions logon: '{:?}', versions: '{:?}'",
            variable_name,
            struct_name,
            tags.logon_versions(),
            tags.versions()
        );
    }

    pub fn get_definer_field_value(
        &self,
        definer_name: &str,
        field_name: &str,
        tags: &Tags,
    ) -> u64 {
        if let Some(e) = self
            .all_definers()
            .find(|a| a.name() == definer_name && a.tags().has_version_intersections(tags))
        {
            for field in e.fields() {
                if field.name() == field_name {
                    return field.value().int();
                }
            }
        }

        panic!(
            "field not found: '{field_name}' in definer: '{definer_name}'",
            field_name = field_name,
            definer_name = definer_name
        )
    }

    pub fn add_vecs(&mut self, mut c: Objects) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
    }

    pub fn print_stats_for_1_12(&self) {
        stats_for_1_12(self);
    }
}
