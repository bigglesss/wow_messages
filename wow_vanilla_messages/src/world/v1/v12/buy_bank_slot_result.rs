use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuyBankSlotResult {
    FAILED_TOO_MANY,
    INSUFFICIENT_FUNDS,
    NOTBANKER,
    OK,
}

impl BuyBankSlotResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::FAILED_TOO_MANY => 0x0,
            Self::INSUFFICIENT_FUNDS => 0x1,
            Self::NOTBANKER => 0x2,
            Self::OK => 0x3,
        }
    }

}

impl Default for BuyBankSlotResult {
    fn default() -> Self {
        Self::FAILED_TOO_MANY
    }
}

impl std::fmt::Display for BuyBankSlotResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FAILED_TOO_MANY => f.write_str("FAILED_TOO_MANY"),
            Self::INSUFFICIENT_FUNDS => f.write_str("INSUFFICIENT_FUNDS"),
            Self::NOTBANKER => f.write_str("NOTBANKER"),
            Self::OK => f.write_str("OK"),
        }
    }
}

impl TryFrom<u32> for BuyBankSlotResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FAILED_TOO_MANY),
            1 => Ok(Self::INSUFFICIENT_FUNDS),
            2 => Ok(Self::NOTBANKER),
            3 => Ok(Self::OK),
            v => Err(crate::errors::EnumError::new("BuyBankSlotResult", v as u32),)
        }
    }
}

