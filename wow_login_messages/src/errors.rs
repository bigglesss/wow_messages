use std::fmt::{format, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    Enum(EnumError),
    String(std::string::FromUtf8Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(i) => i.fmt(f),
            ParseError::Enum(i) => i.fmt(f),
            ParseError::String(i) => i.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<EnumError> for ParseError {
    fn from(e: EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub struct EnumError {
    pub name: &'static str,
    pub value: u32,
}

impl EnumError {
    pub fn new(name: &'static str, value: u32) -> Self {
        Self { name, value }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Enum {} can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl std::error::Error for EnumError {}
