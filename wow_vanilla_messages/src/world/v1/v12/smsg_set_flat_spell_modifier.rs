use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SET_FLAT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl ServerMessageWrite for SMSG_SET_FLAT_SPELL_MODIFIER {}

impl MessageBody for SMSG_SET_FLAT_SPELL_MODIFIER {
    const OPCODE: u16 = 0x0266;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // eff: u8
        let eff = crate::util::read_u8_le(r)?;

        // op: u8
        let op = crate::util::read_u8_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

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
            // eff: u8
            let eff = crate::util::tokio_read_u8_le(r).await?;

            // op: u8
            let op = crate::util::tokio_read_u8_le(r).await?;

            // value: u32
            let value = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                eff,
                op,
                value,
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
            // eff: u8
            w.write_all(&self.eff.to_le_bytes()).await?;

            // op: u8
            w.write_all(&self.op.to_le_bytes()).await?;

            // value: u32
            w.write_all(&self.value.to_le_bytes()).await?;

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
            // eff: u8
            let eff = crate::util::astd_read_u8_le(r).await?;

            // op: u8
            let op = crate::util::astd_read_u8_le(r).await?;

            // value: u32
            let value = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                eff,
                op,
                value,
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
            // eff: u8
            w.write_all(&self.eff.to_le_bytes()).await?;

            // op: u8
            w.write_all(&self.op.to_le_bytes()).await?;

            // value: u32
            w.write_all(&self.value.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_SET_FLAT_SPELL_MODIFIER {}

impl MaximumPossibleSized for SMSG_SET_FLAT_SPELL_MODIFIER {
    fn maximum_possible_size() -> usize {
        0
        + 1 // eff: u8
        + 1 // op: u8
        + 4 // value: u32
    }
}

