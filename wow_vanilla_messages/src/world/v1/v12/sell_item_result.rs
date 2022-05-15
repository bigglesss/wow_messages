use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SellItemResult {
    CANT_FIND_ITEM,
    CANT_SELL_ITEM,
    CANT_FIND_VENDOR,
    YOU_DONT_OWN_THAT_ITEM,
    UNK,
    ONLY_EMPTY_BAG,
}

impl SellItemResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CANT_FIND_ITEM => 0x1,
            Self::CANT_SELL_ITEM => 0x2,
            Self::CANT_FIND_VENDOR => 0x3,
            Self::YOU_DONT_OWN_THAT_ITEM => 0x4,
            Self::UNK => 0x5,
            Self::ONLY_EMPTY_BAG => 0x6,
        }
    }

}

impl Default for SellItemResult {
    fn default() -> Self {
        Self::CANT_FIND_ITEM
    }
}

impl std::fmt::Display for SellItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CANT_FIND_ITEM => f.write_str("CANT_FIND_ITEM"),
            Self::CANT_SELL_ITEM => f.write_str("CANT_SELL_ITEM"),
            Self::CANT_FIND_VENDOR => f.write_str("CANT_FIND_VENDOR"),
            Self::YOU_DONT_OWN_THAT_ITEM => f.write_str("YOU_DONT_OWN_THAT_ITEM"),
            Self::UNK => f.write_str("UNK"),
            Self::ONLY_EMPTY_BAG => f.write_str("ONLY_EMPTY_BAG"),
        }
    }
}

impl TryFrom<u8> for SellItemResult {
    type Error = SellItemResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CANT_FIND_ITEM),
            2 => Ok(Self::CANT_SELL_ITEM),
            3 => Ok(Self::CANT_FIND_VENDOR),
            4 => Ok(Self::YOU_DONT_OWN_THAT_ITEM),
            5 => Ok(Self::UNK),
            6 => Ok(Self::ONLY_EMPTY_BAG),
            _ => Err(SellItemResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct SellItemResultError {
    pub value: u8,
}

impl SellItemResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for SellItemResultError {}
impl std::fmt::Display for SellItemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SellItemResult': '{}'", self.value))
    }
}

