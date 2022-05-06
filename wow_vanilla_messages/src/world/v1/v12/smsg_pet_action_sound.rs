use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetTalkReason, PetTalkReasonError};
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
pub struct SMSG_PET_ACTION_SOUND {
    pub guid: Guid,
    pub reason: PetTalkReason,
}

impl ServerMessageWrite for SMSG_PET_ACTION_SOUND {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PET_ACTION_SOUND {
    const OPCODE: u16 = 0x0324;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PET_ACTION_SOUNDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // reason: PetTalkReason
        let reason = PetTalkReason::read(r)?;

        Ok(Self {
            guid,
            reason,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // reason: PetTalkReason
        self.reason.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // reason: PetTalkReason
        let reason = PetTalkReason::tokio_read(r).await?;

        Ok(Self {
            guid,
            reason,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // reason: PetTalkReason
        self.reason.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // reason: PetTalkReason
        let reason = PetTalkReason::astd_read(r).await?;

        Ok(Self {
            guid,
            reason,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // reason: PetTalkReason
        self.reason.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_PET_ACTION_SOUND {}

impl MaximumPossibleSized for SMSG_PET_ACTION_SOUND {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // reason: PetTalkReason
    }
}

#[derive(Debug)]
pub enum SMSG_PET_ACTION_SOUNDError {
    Io(std::io::Error),
    PetTalkReason(PetTalkReasonError),
}

impl std::error::Error for SMSG_PET_ACTION_SOUNDError {}
impl std::fmt::Display for SMSG_PET_ACTION_SOUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetTalkReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_ACTION_SOUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetTalkReasonError> for SMSG_PET_ACTION_SOUNDError {
    fn from(e: PetTalkReasonError) -> Self {
        Self::PetTalkReason(e)
    }
}

