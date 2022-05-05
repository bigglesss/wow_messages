use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_from: u32,
}

impl ClientMessageWrite for CMSG_AUCTION_LIST_OWNER_ITEMS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_AUCTION_LIST_OWNER_ITEMS {
    const OPCODE: u16 = 0x0259;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // list_from: u32
        let list_from = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_from,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::tokio_read(r).await?;

        // list_from: u32
        let list_from = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            auctioneer_guid,
            list_from,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.tokio_write(w).await?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::astd_read(r).await?;

        // list_from: u32
        let list_from = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            auctioneer_guid,
            list_from,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.astd_write(w).await?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_AUCTION_LIST_OWNER_ITEMS {}

impl MaximumPossibleSized for CMSG_AUCTION_LIST_OWNER_ITEMS {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: Guid
        + 4 // list_from: u32
    }
}

