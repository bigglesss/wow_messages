use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_BUY_ITEM {
    pub guid: Guid,
    pub vendor_slot: u32,
    pub amount_for_sale: u32,
    pub amount_bought: u32,
}

impl ServerMessageWrite for SMSG_BUY_ITEM {}

impl SMSG_BUY_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes())?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_BUY_ITEM {
    const OPCODE: u16 = 0x01a4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // vendor_slot: u32
        let vendor_slot = crate::util::read_u32_le(r)?;

        // amount_for_sale: u32
        let amount_for_sale = crate::util::read_u32_le(r)?;

        // amount_bought: u32
        let amount_bought = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            vendor_slot,
            amount_for_sale,
            amount_bought,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
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

            // vendor_slot: u32
            let vendor_slot = crate::util::tokio_read_u32_le(r).await?;

            // amount_for_sale: u32
            let amount_for_sale = crate::util::tokio_read_u32_le(r).await?;

            // amount_bought: u32
            let amount_bought = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                vendor_slot,
                amount_for_sale,
                amount_bought,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
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

            // vendor_slot: u32
            let vendor_slot = crate::util::astd_read_u32_le(r).await?;

            // amount_for_sale: u32
            let amount_for_sale = crate::util::astd_read_u32_le(r).await?;

            // amount_bought: u32
            let amount_bought = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                vendor_slot,
                amount_for_sale,
                amount_bought,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_BUY_ITEM {
    pub(crate) fn size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // vendor_slot: u32
        + 4 // amount_for_sale: u32
        + 4 // amount_bought: u32
    }
}

