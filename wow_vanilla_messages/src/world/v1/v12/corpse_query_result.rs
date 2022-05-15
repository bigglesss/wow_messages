use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum CorpseQueryResult {
    NOT_FOUND,
    FOUND,
}

impl CorpseQueryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_FOUND => 0x0,
            Self::FOUND => 0x1,
        }
    }

}

impl ConstantSized for CorpseQueryResult {}

impl MaximumPossibleSized for CorpseQueryResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for CorpseQueryResult {
    fn default() -> Self {
        Self::NOT_FOUND
    }
}

impl std::fmt::Display for CorpseQueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_FOUND => f.write_str("NOT_FOUND"),
            Self::FOUND => f.write_str("FOUND"),
        }
    }
}

impl TryFrom<u8> for CorpseQueryResult {
    type Error = CorpseQueryResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_FOUND),
            1 => Ok(Self::FOUND),
            _ => Err(CorpseQueryResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct CorpseQueryResultError {
    value: u8,
}

impl CorpseQueryResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for CorpseQueryResultError {}
impl std::fmt::Display for CorpseQueryResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'CorpseQueryResult': '{}'", self.value))
    }
}

