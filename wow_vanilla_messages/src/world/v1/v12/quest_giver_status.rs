use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L36):
/// ```text
/// enum QuestGiverStatus : u8 {
///     NONE = 0;
///     UNAVAILABLE = 1;
///     CHAT = 2;
///     INCOMPLETE = 3;
///     REWARD_REP = 4;
///     AVAILABLE = 5;
///     REWARD_OLD = 6;
///     REWARD2 = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestGiverStatus {
    NONE,
    UNAVAILABLE,
    CHAT,
    INCOMPLETE,
    REWARD_REP,
    AVAILABLE,
    REWARD_OLD,
    REWARD2,
}

impl ReadableAndWritable for QuestGiverStatus {
    type Error = QuestGiverStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl QuestGiverStatus {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestGiverStatusError> {
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
            Self::UNAVAILABLE => 0x1,
            Self::CHAT => 0x2,
            Self::INCOMPLETE => 0x3,
            Self::REWARD_REP => 0x4,
            Self::AVAILABLE => 0x5,
            Self::REWARD_OLD => 0x6,
            Self::REWARD2 => 0x7,
        }
    }

    pub const fn new() -> Self {
        Self::NONE
    }

}

impl ConstantSized for QuestGiverStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for QuestGiverStatus {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for QuestGiverStatus {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for QuestGiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::UNAVAILABLE => f.write_str("UNAVAILABLE"),
            Self::CHAT => f.write_str("CHAT"),
            Self::INCOMPLETE => f.write_str("INCOMPLETE"),
            Self::REWARD_REP => f.write_str("REWARD_REP"),
            Self::AVAILABLE => f.write_str("AVAILABLE"),
            Self::REWARD_OLD => f.write_str("REWARD_OLD"),
            Self::REWARD2 => f.write_str("REWARD2"),
        }
    }
}

impl TryFrom<u8> for QuestGiverStatus {
    type Error = TryFromQuestGiverStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::UNAVAILABLE),
            2 => Ok(Self::CHAT),
            3 => Ok(Self::INCOMPLETE),
            4 => Ok(Self::REWARD_REP),
            5 => Ok(Self::AVAILABLE),
            6 => Ok(Self::REWARD_OLD),
            7 => Ok(Self::REWARD2),
            _ => Err(TryFromQuestGiverStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromQuestGiverStatusError {
    value: u8,
}

impl TryFromQuestGiverStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum QuestGiverStatusError {
    Read(std::io::Error),
    TryFrom(TryFromQuestGiverStatusError),
}

impl std::error::Error for QuestGiverStatusError {}
impl std::fmt::Display for TryFromQuestGiverStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestGiverStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for QuestGiverStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for QuestGiverStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromQuestGiverStatusError> for QuestGiverStatusError {
    fn from(value: TryFromQuestGiverStatusError) -> Self {
        Self::TryFrom(value)
    }
}
