use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SPELLINSTAKILLLOG {
    pub target_guid: Guid,
    pub spell: u32,
}

impl ServerMessageWrite for SMSG_SPELLINSTAKILLLOG {}

impl MessageBody for SMSG_SPELLINSTAKILLLOG {
    const OPCODE: u16 = 0x032f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            target_guid,
            spell,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }

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
            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                target_guid,
                spell,
            })
        })
    }

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
            // target_guid: Guid
            self.target_guid.tokio_write(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            Ok(())
        })
    }

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
            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                target_guid,
                spell,
            })
        })
    }

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
            // target_guid: Guid
            self.target_guid.astd_write(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_SPELLINSTAKILLLOG {}

impl MaximumPossibleSized for SMSG_SPELLINSTAKILLLOG {
    fn maximum_possible_size() -> usize {
        0
        + 8 // target_guid: Guid
        + 4 // spell: u32
    }
}

