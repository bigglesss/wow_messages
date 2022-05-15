use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ExperienceAwardType {
    KILL,
    NON_KILL,
}

impl ExperienceAwardType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::KILL => 0x0,
            Self::NON_KILL => 0x1,
        }
    }

}

impl Default for ExperienceAwardType {
    fn default() -> Self {
        Self::KILL
    }
}

impl std::fmt::Display for ExperienceAwardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KILL => f.write_str("KILL"),
            Self::NON_KILL => f.write_str("NON_KILL"),
        }
    }
}

impl TryFrom<u8> for ExperienceAwardType {
    type Error = ExperienceAwardTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::KILL),
            1 => Ok(Self::NON_KILL),
            _ => Err(ExperienceAwardTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ExperienceAwardTypeError {
    value: u8,
}

impl ExperienceAwardTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ExperienceAwardTypeError {}
impl std::fmt::Display for ExperienceAwardTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ExperienceAwardType': '{}'", self.value))
    }
}

