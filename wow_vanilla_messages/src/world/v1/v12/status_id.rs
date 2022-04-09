use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L3):
/// ```text
/// enum StatusId : u8 {
///     NONE = 0;
///     WAIT_QUEUE = 1;
///     WAIT_JOIN = 2;
///     IN_PROGRESS = 3;
///     WAIT_LEAVE = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum StatusId {
    NONE,
    WAIT_QUEUE,
    WAIT_JOIN,
    IN_PROGRESS,
    WAIT_LEAVE,
}

impl ReadableAndWritable for StatusId {
    type Error = StatusIdError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl StatusId {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StatusIdError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::WAIT_QUEUE => 0x1,
            Self::WAIT_JOIN => 0x2,
            Self::IN_PROGRESS => 0x3,
            Self::WAIT_LEAVE => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::NONE
    }

}

impl ConstantSized for StatusId {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for StatusId {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for StatusId {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::WAIT_QUEUE => f.write_str("WAIT_QUEUE"),
            Self::WAIT_JOIN => f.write_str("WAIT_JOIN"),
            Self::IN_PROGRESS => f.write_str("IN_PROGRESS"),
            Self::WAIT_LEAVE => f.write_str("WAIT_LEAVE"),
        }
    }
}

impl TryFrom<u8> for StatusId {
    type Error = TryFromStatusIdError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::WAIT_QUEUE),
            2 => Ok(Self::WAIT_JOIN),
            3 => Ok(Self::IN_PROGRESS),
            4 => Ok(Self::WAIT_LEAVE),
            _ => Err(TryFromStatusIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromStatusIdError {
    value: u8,
}

impl TryFromStatusIdError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum StatusIdError {
    Read(std::io::Error),
    TryFrom(TryFromStatusIdError),
}

impl std::error::Error for StatusIdError {}
impl std::fmt::Display for TryFromStatusIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'StatusId': '{}'", self.value))
    }
}

impl std::fmt::Display for StatusIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for StatusIdError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromStatusIdError> for StatusIdError {
    fn from(value: TryFromStatusIdError) -> Self {
        Self::TryFrom(value)
    }
}
