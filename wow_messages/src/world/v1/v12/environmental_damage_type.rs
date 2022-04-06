use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_environmentaldamagelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_environmentaldamagelog.wowm#L3):
/// ```text
/// enum EnvironmentalDamageType : u32 {
///     EXHAUSTED = 0;
///     DROWNING = 1;
///     FALL = 2;
///     LAVA = 3;
///     SLIME = 4;
///     FIRE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum EnvironmentalDamageType {
    EXHAUSTED,
    DROWNING,
    FALL,
    LAVA,
    SLIME,
    FIRE,
}

impl ReadableAndWritable for EnvironmentalDamageType {
    type Error = EnvironmentalDamageTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl EnvironmentalDamageType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EnvironmentalDamageTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EnvironmentalDamageTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, EnvironmentalDamageTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::EXHAUSTED => 0x0,
            Self::DROWNING => 0x1,
            Self::FALL => 0x2,
            Self::LAVA => 0x3,
            Self::SLIME => 0x4,
            Self::FIRE => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::EXHAUSTED
    }

}

impl ConstantSized for EnvironmentalDamageType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for EnvironmentalDamageType {
    fn maximum_possible_size() -> usize {
        4
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
    type Error = TryFromEnvironmentalDamageTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EXHAUSTED),
            1 => Ok(Self::DROWNING),
            2 => Ok(Self::FALL),
            3 => Ok(Self::LAVA),
            4 => Ok(Self::SLIME),
            5 => Ok(Self::FIRE),
            _ => Err(TryFromEnvironmentalDamageTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromEnvironmentalDamageTypeError {
    value: u32,
}

impl TryFromEnvironmentalDamageTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum EnvironmentalDamageTypeError {
    Read(std::io::Error),
    TryFrom(TryFromEnvironmentalDamageTypeError),
}

impl std::error::Error for EnvironmentalDamageTypeError {}
impl std::fmt::Display for TryFromEnvironmentalDamageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'EnvironmentalDamageType': '{}'", self.value))
    }
}

impl std::fmt::Display for EnvironmentalDamageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for EnvironmentalDamageTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromEnvironmentalDamageTypeError> for EnvironmentalDamageTypeError {
    fn from(value: TryFromEnvironmentalDamageTypeError) -> Self {
        Self::TryFrom(value)
    }
}

