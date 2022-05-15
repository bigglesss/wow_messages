use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_AUCTION_REMOVE_ITEM {
    pub auctioneer_guid: Guid,
    pub auction_id: u32,
}

impl ClientMessageWrite for CMSG_AUCTION_REMOVE_ITEM {}

impl MessageBody for CMSG_AUCTION_REMOVE_ITEM {
    const OPCODE: u16 = 0x0257;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            auction_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::tokio_read(r).await?;

            // auction_id: u32
            let auction_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                auction_id,
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
            // auctioneer_guid: Guid
            self.auctioneer_guid.tokio_write(w).await?;

            // auction_id: u32
            w.write_all(&self.auction_id.to_le_bytes()).await?;

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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::astd_read(r).await?;

            // auction_id: u32
            let auction_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                auction_id,
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
            // auctioneer_guid: Guid
            self.auctioneer_guid.astd_write(w).await?;

            // auction_id: u32
            w.write_all(&self.auction_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_AUCTION_REMOVE_ITEM {}

impl MaximumPossibleSized for CMSG_AUCTION_REMOVE_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // auctioneer_guid: Guid
        + 4 // auction_id: u32
    }
}

