use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetitionResult, PetitionResultError};
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
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition_guid: Guid,
    pub owner_guid: Guid,
    pub result: PetitionResult,
}

impl ServerMessageWrite for SMSG_PETITION_SIGN_RESULTS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u16 = 0x01c1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PETITION_SIGN_RESULTSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // result: PetitionResult
        let result = PetitionResult::read(r)?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // owner_guid: Guid
        self.owner_guid.write(w)?;

        // result: PetitionResult
        self.result.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::tokio_read(r).await?;

        // owner_guid: Guid
        let owner_guid = Guid::tokio_read(r).await?;

        // result: PetitionResult
        let result = PetitionResult::tokio_read(r).await?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.tokio_write(w).await?;

        // owner_guid: Guid
        self.owner_guid.tokio_write(w).await?;

        // result: PetitionResult
        self.result.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::astd_read(r).await?;

        // owner_guid: Guid
        let owner_guid = Guid::astd_read(r).await?;

        // result: PetitionResult
        let result = PetitionResult::astd_read(r).await?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.astd_write(w).await?;

        // owner_guid: Guid
        self.owner_guid.astd_write(w).await?;

        // result: PetitionResult
        self.result.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_PETITION_SIGN_RESULTS {}

impl MaximumPossibleSized for SMSG_PETITION_SIGN_RESULTS {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: Guid
        + 8 // owner_guid: Guid
        + PetitionResult::size() // result: PetitionResult
    }
}

#[derive(Debug)]
pub enum SMSG_PETITION_SIGN_RESULTSError {
    Io(std::io::Error),
    PetitionResult(PetitionResultError),
}

impl std::error::Error for SMSG_PETITION_SIGN_RESULTSError {}
impl std::fmt::Display for SMSG_PETITION_SIGN_RESULTSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetitionResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetitionResultError> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e: PetitionResultError) -> Self {
        Self::PetitionResult(e)
    }
}

