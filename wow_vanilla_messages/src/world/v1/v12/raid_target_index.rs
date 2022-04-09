use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L8):
/// ```text
/// enum RaidTargetIndex : u8 {
///     UNKNOWN0 = 0;
///     UNKNOWN1 = 1;
///     UNKNOWN2 = 2;
///     UNKNOWN3 = 3;
///     UNKNOWN4 = 4;
///     UNKNOWN5 = 5;
///     UNKNOWN6 = 6;
///     UNKNOWN7 = 7;
///     UNKNOWN8 = 8;
///     REQUEST_ICONS = 0xFF;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidTargetIndex {
    UNKNOWN0,
    UNKNOWN1,
    UNKNOWN2,
    UNKNOWN3,
    UNKNOWN4,
    UNKNOWN5,
    UNKNOWN6,
    UNKNOWN7,
    UNKNOWN8,
    REQUEST_ICONS,
}

impl ReadableAndWritable for RaidTargetIndex {
    type Error = RaidTargetIndexError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl RaidTargetIndex {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetIndexError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::UNKNOWN0 => 0x0,
            Self::UNKNOWN1 => 0x1,
            Self::UNKNOWN2 => 0x2,
            Self::UNKNOWN3 => 0x3,
            Self::UNKNOWN4 => 0x4,
            Self::UNKNOWN5 => 0x5,
            Self::UNKNOWN6 => 0x6,
            Self::UNKNOWN7 => 0x7,
            Self::UNKNOWN8 => 0x8,
            Self::REQUEST_ICONS => 0xff,
        }
    }

    pub const fn new() -> Self {
        Self::UNKNOWN0
    }

}

impl ConstantSized for RaidTargetIndex {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidTargetIndex {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for RaidTargetIndex {
    fn default() -> Self {
        Self::UNKNOWN0
    }
}

impl std::fmt::Display for RaidTargetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNKNOWN0 => f.write_str("UNKNOWN0"),
            Self::UNKNOWN1 => f.write_str("UNKNOWN1"),
            Self::UNKNOWN2 => f.write_str("UNKNOWN2"),
            Self::UNKNOWN3 => f.write_str("UNKNOWN3"),
            Self::UNKNOWN4 => f.write_str("UNKNOWN4"),
            Self::UNKNOWN5 => f.write_str("UNKNOWN5"),
            Self::UNKNOWN6 => f.write_str("UNKNOWN6"),
            Self::UNKNOWN7 => f.write_str("UNKNOWN7"),
            Self::UNKNOWN8 => f.write_str("UNKNOWN8"),
            Self::REQUEST_ICONS => f.write_str("REQUEST_ICONS"),
        }
    }
}

impl TryFrom<u8> for RaidTargetIndex {
    type Error = TryFromRaidTargetIndexError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNKNOWN0),
            1 => Ok(Self::UNKNOWN1),
            2 => Ok(Self::UNKNOWN2),
            3 => Ok(Self::UNKNOWN3),
            4 => Ok(Self::UNKNOWN4),
            5 => Ok(Self::UNKNOWN5),
            6 => Ok(Self::UNKNOWN6),
            7 => Ok(Self::UNKNOWN7),
            8 => Ok(Self::UNKNOWN8),
            255 => Ok(Self::REQUEST_ICONS),
            _ => Err(TryFromRaidTargetIndexError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRaidTargetIndexError {
    value: u8,
}

impl TryFromRaidTargetIndexError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RaidTargetIndexError {
    Read(std::io::Error),
    TryFrom(TryFromRaidTargetIndexError),
}

impl std::error::Error for RaidTargetIndexError {}
impl std::fmt::Display for TryFromRaidTargetIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidTargetIndex': '{}'", self.value))
    }
}

impl std::fmt::Display for RaidTargetIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidTargetIndexError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRaidTargetIndexError> for RaidTargetIndexError {
    fn from(value: TryFromRaidTargetIndexError) -> Self {
        Self::TryFrom(value)
    }
}
