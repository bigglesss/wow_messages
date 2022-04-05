use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L3):
/// ```text
/// enum MailType : u8 {
///     NORMAL = 0;
///     AUCTION = 2;
///     CREATURE = 3;
///     GAMEOBJECT = 4;
///     ITEM = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MailType {
    NORMAL,
    AUCTION,
    CREATURE,
    GAMEOBJECT,
    ITEM,
}

impl ReadableAndWritable for MailType {
    type Error = MailTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl MailType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::AUCTION => 0x2,
            Self::CREATURE => 0x3,
            Self::GAMEOBJECT => 0x4,
            Self::ITEM => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::NORMAL
    }

}

impl ConstantSized for MailType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MailType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for MailType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for MailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::AUCTION => f.write_str("AUCTION"),
            Self::CREATURE => f.write_str("CREATURE"),
            Self::GAMEOBJECT => f.write_str("GAMEOBJECT"),
            Self::ITEM => f.write_str("ITEM"),
        }
    }
}

impl TryFrom<u8> for MailType {
    type Error = TryFromMailTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            2 => Ok(Self::AUCTION),
            3 => Ok(Self::CREATURE),
            4 => Ok(Self::GAMEOBJECT),
            5 => Ok(Self::ITEM),
            _ => Err(TryFromMailTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMailTypeError {
    value: u8,
}

impl TryFromMailTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MailTypeError {
    Read(std::io::Error),
    TryFrom(TryFromMailTypeError),
}

impl std::error::Error for MailTypeError {}
impl std::fmt::Display for TryFromMailTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MailType': '{}'", self.value))
    }
}

impl std::fmt::Display for MailTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MailTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMailTypeError> for MailTypeError {
    fn from(value: TryFromMailTypeError) -> Self {
        Self::TryFrom(value)
    }
}

