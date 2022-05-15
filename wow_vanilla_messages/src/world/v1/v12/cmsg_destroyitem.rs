use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_DESTROYITEM {
    pub bag: u8,
    pub slot: u8,
    pub amount: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}

impl ClientMessageWrite for CMSG_DESTROYITEM {}

impl MessageBody for CMSG_DESTROYITEM {
    const OPCODE: u16 = 0x0111;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag: u8
        let bag = crate::util::read_u8_le(r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag,
            slot,
            amount,
            unknown1,
            unknown2,
            unknown3,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

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
            // bag: u8
            let bag = crate::util::tokio_read_u8_le(r).await?;

            // slot: u8
            let slot = crate::util::tokio_read_u8_le(r).await?;

            // amount: u8
            let amount = crate::util::tokio_read_u8_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            // unknown2: u8
            let unknown2 = crate::util::tokio_read_u8_le(r).await?;

            // unknown3: u8
            let unknown3 = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                bag,
                slot,
                amount,
                unknown1,
                unknown2,
                unknown3,
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
            // bag: u8
            w.write_all(&self.bag.to_le_bytes()).await?;

            // slot: u8
            w.write_all(&self.slot.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // unknown2: u8
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // unknown3: u8
            w.write_all(&self.unknown3.to_le_bytes()).await?;

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
            // bag: u8
            let bag = crate::util::astd_read_u8_le(r).await?;

            // slot: u8
            let slot = crate::util::astd_read_u8_le(r).await?;

            // amount: u8
            let amount = crate::util::astd_read_u8_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            // unknown2: u8
            let unknown2 = crate::util::astd_read_u8_le(r).await?;

            // unknown3: u8
            let unknown3 = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                bag,
                slot,
                amount,
                unknown1,
                unknown2,
                unknown3,
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
            // bag: u8
            w.write_all(&self.bag.to_le_bytes()).await?;

            // slot: u8
            w.write_all(&self.slot.to_le_bytes()).await?;

            // amount: u8
            w.write_all(&self.amount.to_le_bytes()).await?;

            // unknown1: u8
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // unknown2: u8
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // unknown3: u8
            w.write_all(&self.unknown3.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_DESTROYITEM {}

impl MaximumPossibleSized for CMSG_DESTROYITEM {
    fn maximum_possible_size() -> usize {
        0
        + 1 // bag: u8
        + 1 // slot: u8
        + 1 // amount: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
    }
}

