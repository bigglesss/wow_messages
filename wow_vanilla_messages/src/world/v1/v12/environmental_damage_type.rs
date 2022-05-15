use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum EnvironmentalDamageType {
    EXHAUSTED,
    DROWNING,
    FALL,
    LAVA,
    SLIME,
    FIRE,
}

impl EnvironmentalDamageType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::EXHAUSTED => 0x0,
            Self::DROWNING => 0x1,
            Self::FALL => 0x2,
            Self::LAVA => 0x3,
            Self::SLIME => 0x4,
            Self::FIRE => 0x5,
        }
    }

}

impl Default for EnvironmentalDamageType {
    fn default() -> Self {
        Self::EXHAUSTED
    }
}

impl std::fmt::Display for EnvironmentalDamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EXHAUSTED => f.write_str("EXHAUSTED"),
            Self::DROWNING => f.write_str("DROWNING"),
            Self::FALL => f.write_str("FALL"),
            Self::LAVA => f.write_str("LAVA"),
            Self::SLIME => f.write_str("SLIME"),
            Self::FIRE => f.write_str("FIRE"),
        }
    }
}

impl TryFrom<u32> for EnvironmentalDamageType {
    type Error = EnvironmentalDamageTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EXHAUSTED),
            1 => Ok(Self::DROWNING),
            2 => Ok(Self::FALL),
            3 => Ok(Self::LAVA),
            4 => Ok(Self::SLIME),
            5 => Ok(Self::FIRE),
            _ => Err(EnvironmentalDamageTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentalDamageTypeError {
    value: u32,
}

impl EnvironmentalDamageTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for EnvironmentalDamageTypeError {}
impl std::fmt::Display for EnvironmentalDamageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'EnvironmentalDamageType': '{}'", self.value))
    }
}

