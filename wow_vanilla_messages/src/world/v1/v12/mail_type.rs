use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum MailType {
    NORMAL,
    AUCTION,
    CREATURE,
    GAMEOBJECT,
    ITEM,
}

impl MailType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::AUCTION => 0x2,
            Self::CREATURE => 0x3,
            Self::GAMEOBJECT => 0x4,
            Self::ITEM => 0x5,
        }
    }

}

impl Default for MailType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for MailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::AUCTION => f.write_str("AUCTION"),
            Self::CREATURE => f.write_str("CREATURE"),
            Self::GAMEOBJECT => f.write_str("GAMEOBJECT"),
            Self::ITEM => f.write_str("ITEM"),
        }
    }
}

impl TryFrom<u8> for MailType {
    type Error = MailTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            2 => Ok(Self::AUCTION),
            3 => Ok(Self::CREATURE),
            4 => Ok(Self::GAMEOBJECT),
            5 => Ok(Self::ITEM),
            _ => Err(MailTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct MailTypeError {
    pub value: u8,
}

impl MailTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for MailTypeError {}
impl std::fmt::Display for MailTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MailType': '{}'", self.value))
    }
}

