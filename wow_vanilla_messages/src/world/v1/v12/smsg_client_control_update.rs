use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CLIENT_CONTROL_UPDATE {
    pub guid: Guid,
    pub allow_movement: u8,
}

impl ServerMessageWrite for SMSG_CLIENT_CONTROL_UPDATE {}

impl MessageBody for SMSG_CLIENT_CONTROL_UPDATE {
    const OPCODE: u16 = 0x0159;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // allow_movement: u8
        let allow_movement = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            allow_movement,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // allow_movement: u8
        w.write_all(&self.allow_movement.to_le_bytes())?;

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
            // guid: PackedGuid
            let guid = Guid::tokio_read_packed(r).await?;

            // allow_movement: u8
            let allow_movement = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                allow_movement,
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
            // guid: PackedGuid
            self.guid.tokio_write_packed(w).await?;

            // allow_movement: u8
            w.write_all(&self.allow_movement.to_le_bytes()).await?;

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
            // guid: PackedGuid
            let guid = Guid::astd_read_packed(r).await?;

            // allow_movement: u8
            let allow_movement = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                allow_movement,
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
            // guid: PackedGuid
            self.guid.astd_write_packed(w).await?;

            // allow_movement: u8
            w.write_all(&self.allow_movement.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_CLIENT_CONTROL_UPDATE {
    fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
        + 1 // allow_movement: u8
    }
}

impl MaximumPossibleSized for SMSG_CLIENT_CONTROL_UPDATE {
    fn maximum_possible_size() -> usize {
        0
        + 9 // guid: Guid
        + 1 // allow_movement: u8
    }
}

