use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{QuestFailedReason, QuestFailedReasonError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_QUESTGIVER_QUEST_FAILED {
    pub quest_id: u32,
    pub reason: QuestFailedReason,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_FAILED {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_QUESTGIVER_QUEST_FAILED {
    const OPCODE: u16 = 0x0192;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_QUESTGIVER_QUEST_FAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // reason: QuestFailedReason
        let reason = QuestFailedReason::read(r)?;

        Ok(Self {
            quest_id,
            reason,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reason: QuestFailedReason
        self.reason.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::tokio_read_u32_le(r).await?;

        // reason: QuestFailedReason
        let reason = QuestFailedReason::tokio_read(r).await?;

        Ok(Self {
            quest_id,
            reason,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // reason: QuestFailedReason
        self.reason.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::astd_read_u32_le(r).await?;

        // reason: QuestFailedReason
        let reason = QuestFailedReason::astd_read(r).await?;

        Ok(Self {
            quest_id,
            reason,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // reason: QuestFailedReason
        self.reason.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_QUESTGIVER_QUEST_FAILED {}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_FAILED {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
        + QuestFailedReason::size() // reason: QuestFailedReason
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_FAILEDError {
    Io(std::io::Error),
    QuestFailedReason(QuestFailedReasonError),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_FAILEDError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestFailedReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestFailedReasonError> for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn from(e: QuestFailedReasonError) -> Self {
        Self::QuestFailedReason(e)
    }
}

