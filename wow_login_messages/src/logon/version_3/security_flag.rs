use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SecurityFlag {
    NONE,
    PIN,
}

impl SecurityFlag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::PIN => 0x1,
        }
    }

}

impl Default for SecurityFlag {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::PIN => f.write_str("PIN"),
        }
    }
}

impl TryFrom<u8> for SecurityFlag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::PIN),
            v => Err(crate::errors::EnumError::new("SecurityFlag", v as u32),)
        }
    }
}

