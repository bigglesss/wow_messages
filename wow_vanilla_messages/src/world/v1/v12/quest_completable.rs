use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L9):
/// ```text
/// enum QuestCompletable : u32 {
///     NOT_COMPLETABLE = 0;
///     COMPLETEABLE = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestCompletable {
    NOT_COMPLETABLE,
    COMPLETEABLE,
}

impl ReadableAndWritable for QuestCompletable {
    type Error = QuestCompletableError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl QuestCompletable {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestCompletableError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestCompletableError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestCompletableError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::NOT_COMPLETABLE => 0x0,
            Self::COMPLETEABLE => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::NOT_COMPLETABLE
    }

}

impl ConstantSized for QuestCompletable {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for QuestCompletable {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for QuestCompletable {
    fn default() -> Self {
        Self::NOT_COMPLETABLE
    }
}

impl std::fmt::Display for QuestCompletable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_COMPLETABLE => f.write_str("NOT_COMPLETABLE"),
            Self::COMPLETEABLE => f.write_str("COMPLETEABLE"),
        }
    }
}

impl TryFrom<u32> for QuestCompletable {
    type Error = TryFromQuestCompletableError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_COMPLETABLE),
            3 => Ok(Self::COMPLETEABLE),
            _ => Err(TryFromQuestCompletableError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromQuestCompletableError {
    value: u32,
}

impl TryFromQuestCompletableError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum QuestCompletableError {
    Read(std::io::Error),
    TryFrom(TryFromQuestCompletableError),
}

impl std::error::Error for QuestCompletableError {}
impl std::fmt::Display for TryFromQuestCompletableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestCompletable': '{}'", self.value))
    }
}

impl std::fmt::Display for QuestCompletableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for QuestCompletableError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromQuestCompletableError> for QuestCompletableError {
    fn from(value: TryFromQuestCompletableError) -> Self {
        Self::TryFrom(value)
    }
}
