use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{LootMethod, LootMethodError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
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
        let loot_method: LootMethod = crate::util::read_u8_le(r)?.try_into()?;

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
        crate::util::write_u8_le(w, self.loot_method.as_int() as u8)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // loot_method: LootMethod
            let loot_method: LootMethod = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                loot_method,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // loot_method: LootMethod
            crate::util::tokio_write_u8_le(w, self.loot_method.as_int() as u8).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // loot_method: LootMethod
            let loot_method: LootMethod = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                loot_method,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // loot_method: LootMethod
            crate::util::astd_write_u8_le(w, self.loot_method.as_int() as u8).await?;

            Ok(())
        })
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

