use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ServerMessageType {
    SHUTDOWN_TIME,
    RESTART_TIME,
    CUSTOM,
    SHUTDOWN_CANCELLED,
    RESTART_CANCELLED,
}

impl ServerMessageType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SHUTDOWN_TIME => 0x1,
            Self::RESTART_TIME => 0x2,
            Self::CUSTOM => 0x3,
            Self::SHUTDOWN_CANCELLED => 0x4,
            Self::RESTART_CANCELLED => 0x5,
        }
    }

}

impl ConstantSized for ServerMessageType {}

impl MaximumPossibleSized for ServerMessageType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for ServerMessageType {
    fn default() -> Self {
        Self::SHUTDOWN_TIME
    }
}

impl std::fmt::Display for ServerMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHUTDOWN_TIME => f.write_str("SHUTDOWN_TIME"),
            Self::RESTART_TIME => f.write_str("RESTART_TIME"),
            Self::CUSTOM => f.write_str("CUSTOM"),
            Self::SHUTDOWN_CANCELLED => f.write_str("SHUTDOWN_CANCELLED"),
            Self::RESTART_CANCELLED => f.write_str("RESTART_CANCELLED"),
        }
    }
}

impl TryFrom<u32> for ServerMessageType {
    type Error = ServerMessageTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SHUTDOWN_TIME),
            2 => Ok(Self::RESTART_TIME),
            3 => Ok(Self::CUSTOM),
            4 => Ok(Self::SHUTDOWN_CANCELLED),
            5 => Ok(Self::RESTART_CANCELLED),
            _ => Err(ServerMessageTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ServerMessageTypeError {
    value: u32,
}

impl ServerMessageTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for ServerMessageTypeError {}
impl std::fmt::Display for ServerMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ServerMessageType': '{}'", self.value))
    }
}

