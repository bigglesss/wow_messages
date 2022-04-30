use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_BUY_ITEM {
    pub guid: Guid,
    pub vendor_slot: u32,
    pub amount_for_sale: u32,
    pub amount_bought: u32,
}

impl ServerMessageWrite for SMSG_BUY_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_BUY_ITEM {
    const OPCODE: u16 = 0x01a4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes())?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes()).await?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes()).await?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes()).await?;

        // amount_for_sale: u32
        w.write_all(&self.amount_for_sale.to_le_bytes()).await?;

        // amount_bought: u32
        w.write_all(&self.amount_bought.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BUY_ITEM {}

impl MaximumPossibleSized for SMSG_BUY_ITEM {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // vendor_slot: u32
        + 4 // amount_for_sale: u32
        + 4 // amount_bought: u32
    }
}

