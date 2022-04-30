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
pub struct SMSG_QUESTGIVER_QUEST_INVALID {
    pub msg: QuestFailedReason,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_INVALID {}

impl MessageBody for SMSG_QUESTGIVER_QUEST_INVALID {
    const OPCODE: u16 = 0x018f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_QUESTGIVER_QUEST_INVALIDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // msg: QuestFailedReason
        let msg = QuestFailedReason::read(r)?;

        Ok(Self {
            msg,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // msg: QuestFailedReason
        self.msg.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTGIVER_QUEST_INVALID {}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_INVALID {
    fn maximum_possible_size() -> usize {
        QuestFailedReason::size() // msg: QuestFailedReason
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_INVALIDError {
    Io(std::io::Error),
    QuestFailedReason(QuestFailedReasonError),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_INVALIDError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_INVALIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestFailedReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_INVALIDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestFailedReasonError> for SMSG_QUESTGIVER_QUEST_INVALIDError {
    fn from(e: QuestFailedReasonError) -> Self {
        Self::QuestFailedReason(e)
    }
}

