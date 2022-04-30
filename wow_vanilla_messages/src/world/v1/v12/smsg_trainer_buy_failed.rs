use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{TrainingFailureReason, TrainingFailureReasonError};
use crate::{WorldServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRAINER_BUY_FAILED {
    pub guid: Guid,
    pub id: u32,
    pub error: TrainingFailureReason,
}

impl WorldServerMessageWrite for SMSG_TRAINER_BUY_FAILED {
    const OPCODE: u16 = 0x1b4;

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
impl MessageBody for SMSG_TRAINER_BUY_FAILED {
    type Error = SMSG_TRAINER_BUY_FAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // error: TrainingFailureReason
        let error = TrainingFailureReason::read(r)?;

        Ok(Self {
            guid,
            id,
            error,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        self.error.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TRAINER_BUY_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TRAINER_BUY_FAILED {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // id: u32
        + TrainingFailureReason::size() // error: TrainingFailureReason
    }
}

#[derive(Debug)]
pub enum SMSG_TRAINER_BUY_FAILEDError {
    Io(std::io::Error),
    TrainingFailureReason(TrainingFailureReasonError),
}

impl std::error::Error for SMSG_TRAINER_BUY_FAILEDError {}
impl std::fmt::Display for SMSG_TRAINER_BUY_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TrainingFailureReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRAINER_BUY_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TrainingFailureReasonError> for SMSG_TRAINER_BUY_FAILEDError {
    fn from(e: TrainingFailureReasonError) -> Self {
        Self::TrainingFailureReason(e)
    }
}

