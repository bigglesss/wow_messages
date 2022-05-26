use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemCreationType {
    RECEIVED,
    CREATED,
}

impl NewItemCreationType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::RECEIVED => 0x0,
            Self::CREATED => 0x1,
        }
    }

}

impl Default for NewItemCreationType {
    fn default() -> Self {
        Self::RECEIVED
    }
}

impl std::fmt::Display for NewItemCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RECEIVED => f.write_str("RECEIVED"),
            Self::CREATED => f.write_str("CREATED"),
        }
    }
}

impl TryFrom<u32> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RECEIVED),
            1 => Ok(Self::CREATED),
            v => Err(crate::errors::EnumError::new("NewItemCreationType", v as u32),)
        }
    }
}

