use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{QuestFailedReason, QuestFailedReasonError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_invalid.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_invalid.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_INVALID = 0x18F {
///     QuestFailedReason msg;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_INVALID {
    pub msg: QuestFailedReason,
}

impl WorldServerMessageWrite for SMSG_QUESTGIVER_QUEST_INVALID {
    const OPCODE: u16 = 0x18f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_QUESTGIVER_QUEST_INVALID {
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

impl ConstantSized for SMSG_QUESTGIVER_QUEST_INVALID {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

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

