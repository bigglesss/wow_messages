use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmType {
    PLAYER_VS_ENVIRONMENT,
    PLAYER_VS_PLAYER,
    ROLEPLAYING,
    ROLEPLAYING_PLAYER_VS_PLAYER,
}

impl RealmType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => 0x0,
            Self::PLAYER_VS_PLAYER => 0x1,
            Self::ROLEPLAYING => 0x6,
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => 0x8,
        }
    }

}

impl ConstantSized for RealmType {}

impl MaximumPossibleSized for RealmType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for RealmType {
    fn default() -> Self {
        Self::PLAYER_VS_ENVIRONMENT
    }
}

impl std::fmt::Display for RealmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => f.write_str("PLAYER_VS_ENVIRONMENT"),
            Self::PLAYER_VS_PLAYER => f.write_str("PLAYER_VS_PLAYER"),
            Self::ROLEPLAYING => f.write_str("ROLEPLAYING"),
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => f.write_str("ROLEPLAYING_PLAYER_VS_PLAYER"),
        }
    }
}

impl TryFrom<u32> for RealmType {
    type Error = RealmTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PLAYER_VS_ENVIRONMENT),
            1 => Ok(Self::PLAYER_VS_PLAYER),
            6 => Ok(Self::ROLEPLAYING),
            8 => Ok(Self::ROLEPLAYING_PLAYER_VS_PLAYER),
            _ => Err(RealmTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RealmTypeError {
    value: u32,
}

impl RealmTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for RealmTypeError {}
impl std::fmt::Display for RealmTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RealmType': '{}'", self.value))
    }
}

