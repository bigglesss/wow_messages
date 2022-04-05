use crate::container::Container;
use crate::file_info::FileInfo;
use crate::parser::types::{
    ArraySize, ArrayType, ObjectType, Objects, Tags, Type, VerifiedContainerValue,
};
use crate::parser::utility::parse_value;

#[derive(Debug, Clone)]
pub struct TestCase {
    subject: String,
    members: Vec<TestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: Tags,
    file_info: FileInfo,
}

impl TestCase {
    pub fn new(
        subject: &str,
        members: Vec<TestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            subject: subject.to_string(),
            members,
            raw_bytes,
            tags,
            file_info,
        }
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn members(&self) -> &[TestCaseMember] {
        &self.members
    }

    pub fn get_member<'a>(t: &'a [TestCaseMember], member: &str) -> &'a TestCaseMember {
        t.iter().find(|a| a.name() == member).unwrap()
    }

    pub fn verify(&mut self, o: &Objects) {
        fn inner(m: &mut TestCaseMember, c: &Container, o: &Objects, tags: &Tags) {
            let ty = c.get_field_ty(&m.variable_name);
            let value = match &mut m.value {
                TestCaseValueInitial::Single(s) => s.clone(),
                TestCaseValueInitial::Multiple(multiple) => {
                    let inner_c = o.get_container(ty.rust_str().as_str(), tags);
                    for m_inner in multiple.iter_mut() {
                        inner(m_inner, inner_c, o, tags);
                    }
                    m.verified_value = Some(TestValue::SubObject {
                        ty_name: ty.rust_str(),
                        members: multiple.to_vec(),
                    });
                    return;
                }
                TestCaseValueInitial::ArrayOfMultiple(array) => {
                    let mut v = Vec::new();

                    let ty_name = match ty {
                        Type::Array(array) => match array.ty() {
                            ArrayType::Integer(_) => panic!(),
                            ArrayType::Complex(c) => c.as_str(),
                            ArrayType::CString => unimplemented!(),
                            ArrayType::Guid => "Guid",
                            ArrayType::PackedGuid => "Guid",
                        },
                        _ => panic!(),
                    };

                    for multiple in array {
                        let mut members = Vec::new();
                        let inner_c = o.get_container(ty_name, tags);

                        for m_inner in multiple {
                            inner(m_inner, inner_c, o, tags);
                            members.push(m_inner.clone());
                        }

                        v.push(members);
                    }

                    m.verified_value = Some(TestValue::ArrayOfSubObject(ty_name.to_string(), v));
                    return;
                }
            };

            let tv = match ty {
                Type::CString | Type::String { .. } => TestValue::String(value.replace('\"', "")),
                Type::Array(array) => {
                    assert!(value.contains('['));
                    assert!(value.contains(']'));
                    let val = &value.replace('[', "").replace(']', "");
                    let mut v = Vec::new();
                    for value in val.split(',') {
                        let value = value.trim();
                        if value.is_empty() {
                            continue;
                        }

                        v.push(parse_value(value).unwrap() as usize);
                    }
                    TestValue::Array {
                        values: v,
                        size: array.size(),
                    }
                }
                Type::FloatingPoint(_) => TestValue::FloatingNumber {
                    value: value.parse().unwrap(),
                    original_string: value.clone(),
                },
                Type::Integer(_) | Type::Guid | Type::PackedGuid => TestValue::Number(
                    VerifiedContainerValue::new(parse_value(&value).unwrap(), value.clone()),
                ),
                Type::Identifier { .. } => {
                    let sub_ty = o.get_object_type_of(ty.rust_str().as_str(), tags);
                    match sub_ty {
                        ObjectType::Flag => {
                            let mut v = Vec::new();
                            for flag in value.split('|') {
                                v.push(flag.trim().to_owned());
                            }
                            TestValue::Flag(v)
                        }
                        _ => {
                            let v = o.get_definer_field_value(ty.rust_str().as_str(), &value, tags);
                            TestValue::Enum(VerifiedContainerValue::new(v, value))
                        }
                    }
                }
                Type::UpdateMask | Type::AuraMask => {
                    panic!("unimplemented")
                }
            };

            m.verified_value = Some(tv);
        }

        let tags = self.tags().clone();
        let c = o.get_container(self.subject(), &tags);

        // Set verified value
        for m in &mut self.members {
            inner(m, c, o, &tags);
        }
    }
}

#[derive(Debug, Clone)]
pub enum TestCaseValueInitial {
    Single(String),
    Multiple(Vec<TestCaseMember>),
    ArrayOfMultiple(Vec<Vec<TestCaseMember>>),
}

#[derive(Debug, Clone)]
pub struct TestCaseMember {
    variable_name: String,
    value: TestCaseValueInitial,
    verified_value: Option<TestValue>,
    tags: Tags,
}

impl TestCaseMember {
    pub fn name(&self) -> &str {
        &self.variable_name
    }

    pub fn value(&self) -> &TestValue {
        self.verified_value.as_ref().unwrap()
    }

    pub fn float_value(&self) -> f64 {
        match self.value() {
            TestValue::FloatingNumber { value, .. } => *value,
            _ => panic!(),
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn new(name: &str, value: TestCaseValueInitial, tags: Tags) -> Self {
        Self {
            variable_name: name.to_string(),
            value,
            verified_value: None,
            tags,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TestValue {
    Number(VerifiedContainerValue),
    FloatingNumber {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<usize>,
        size: ArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(VerifiedContainerValue),
    SubObject {
        ty_name: String,
        members: Vec<TestCaseMember>,
    },
    ArrayOfSubObject(String, Vec<Vec<TestCaseMember>>),
}

impl TestValue {
    pub fn value(&self) -> &VerifiedContainerValue {
        match self {
            TestValue::Number(i) => i,
            _ => panic!(),
        }
    }

    pub fn original_string(&self) -> &str {
        match self {
            TestValue::Number(i) => i.original_string(),
            TestValue::Enum(i) => i.original_string(),
            TestValue::FloatingNumber {
                original_string, ..
            } => original_string.as_str(),
            TestValue::String(s) => s,
            _ => panic!(),
        }
    }
}
