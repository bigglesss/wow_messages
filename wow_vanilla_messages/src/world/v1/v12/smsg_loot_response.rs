use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{LootMethod, LootMethodError};
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
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: LootMethod,
}

impl ServerMessageWrite for SMSG_LOOT_RESPONSE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_LOOT_RESPONSE {
    const OPCODE: u16 = 0x0160;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_LOOT_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // loot_method: LootMethod
        let loot_method = LootMethod::read(r)?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // loot_method: LootMethod
        self.loot_method.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // loot_method: LootMethod
        let loot_method = LootMethod::tokio_read(r).await?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // loot_method: LootMethod
        self.loot_method.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // loot_method: LootMethod
        let loot_method = LootMethod::astd_read(r).await?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // loot_method: LootMethod
        self.loot_method.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_LOOT_RESPONSE {}

impl MaximumPossibleSized for SMSG_LOOT_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // loot_method: LootMethod
    }
}

#[derive(Debug)]
pub enum SMSG_LOOT_RESPONSEError {
    Io(std::io::Error),
    LootMethod(LootMethodError),
}

impl std::error::Error for SMSG_LOOT_RESPONSEError {}
impl std::fmt::Display for SMSG_LOOT_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LootMethod(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOOT_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LootMethodError> for SMSG_LOOT_RESPONSEError {
    fn from(e: LootMethodError) -> Self {
        Self::LootMethod(e)
    }
}

