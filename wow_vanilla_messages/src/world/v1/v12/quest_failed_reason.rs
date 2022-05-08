use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestFailedReason {
    DONT_HAVE_REQ,
    QUEST_FAILED_LOW_LEVEL,
    QUEST_FAILED_REQS,
    QUEST_FAILED_INVENTORY_FULL,
    QUEST_FAILED_WRONG_RACE,
    QUEST_ONLY_ONE_TIMED,
    QUEST_ALREADY_ON,
    QUEST_FAILED_DUPLICATE_ITEM,
    QUEST_FAILED_MISSING_ITEMS,
    QUEST_FAILED_NOT_ENOUGH_MONEY,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for QuestFailedReason {
    type Error = QuestFailedReasonError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u32_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::astd_read_u32_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

}

impl QuestFailedReason {
    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestFailedReasonError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    pub const fn as_int(&self) -> u32 {
        match self {
            Self::DONT_HAVE_REQ => 0x0,
            Self::QUEST_FAILED_LOW_LEVEL => 0x1,
            Self::QUEST_FAILED_REQS => 0x2,
            Self::QUEST_FAILED_INVENTORY_FULL => 0x4,
            Self::QUEST_FAILED_WRONG_RACE => 0x6,
            Self::QUEST_ONLY_ONE_TIMED => 0xc,
            Self::QUEST_ALREADY_ON => 0xd,
            Self::QUEST_FAILED_DUPLICATE_ITEM => 0x11,
            Self::QUEST_FAILED_MISSING_ITEMS => 0x14,
            Self::QUEST_FAILED_NOT_ENOUGH_MONEY => 0x16,
        }
    }

    pub const fn new() -> Self {
        Self::DONT_HAVE_REQ
    }

}

impl ConstantSized for QuestFailedReason {}

impl MaximumPossibleSized for QuestFailedReason {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for QuestFailedReason {
    fn default() -> Self {
        Self::DONT_HAVE_REQ
    }
}

impl std::fmt::Display for QuestFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DONT_HAVE_REQ => f.write_str("DONT_HAVE_REQ"),
            Self::QUEST_FAILED_LOW_LEVEL => f.write_str("QUEST_FAILED_LOW_LEVEL"),
            Self::QUEST_FAILED_REQS => f.write_str("QUEST_FAILED_REQS"),
            Self::QUEST_FAILED_INVENTORY_FULL => f.write_str("QUEST_FAILED_INVENTORY_FULL"),
            Self::QUEST_FAILED_WRONG_RACE => f.write_str("QUEST_FAILED_WRONG_RACE"),
            Self::QUEST_ONLY_ONE_TIMED => f.write_str("QUEST_ONLY_ONE_TIMED"),
            Self::QUEST_ALREADY_ON => f.write_str("QUEST_ALREADY_ON"),
            Self::QUEST_FAILED_DUPLICATE_ITEM => f.write_str("QUEST_FAILED_DUPLICATE_ITEM"),
            Self::QUEST_FAILED_MISSING_ITEMS => f.write_str("QUEST_FAILED_MISSING_ITEMS"),
            Self::QUEST_FAILED_NOT_ENOUGH_MONEY => f.write_str("QUEST_FAILED_NOT_ENOUGH_MONEY"),
        }
    }
}

impl TryFrom<u32> for QuestFailedReason {
    type Error = TryFromQuestFailedReasonError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DONT_HAVE_REQ),
            1 => Ok(Self::QUEST_FAILED_LOW_LEVEL),
            2 => Ok(Self::QUEST_FAILED_REQS),
            4 => Ok(Self::QUEST_FAILED_INVENTORY_FULL),
            6 => Ok(Self::QUEST_FAILED_WRONG_RACE),
            12 => Ok(Self::QUEST_ONLY_ONE_TIMED),
            13 => Ok(Self::QUEST_ALREADY_ON),
            17 => Ok(Self::QUEST_FAILED_DUPLICATE_ITEM),
            20 => Ok(Self::QUEST_FAILED_MISSING_ITEMS),
            22 => Ok(Self::QUEST_FAILED_NOT_ENOUGH_MONEY),
            _ => Err(TryFromQuestFailedReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromQuestFailedReasonError {
    value: u32,
}

impl TryFromQuestFailedReasonError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum QuestFailedReasonError {
    Read(std::io::Error),
    TryFrom(TryFromQuestFailedReasonError),
}

impl std::error::Error for QuestFailedReasonError {}
impl std::fmt::Display for TryFromQuestFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestFailedReason': '{}'", self.value))
    }
}

impl std::fmt::Display for QuestFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for QuestFailedReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromQuestFailedReasonError> for QuestFailedReasonError {
    fn from(value: TryFromQuestFailedReasonError) -> Self {
        Self::TryFrom(value)
    }
}

