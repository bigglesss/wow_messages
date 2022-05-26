use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_from: u32,
}

impl CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_AUCTION_LIST_OWNER_ITEMS {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(12);
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0259;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::tokio_read(r).await?;

            // list_from: u32
            let list_from = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                list_from,
            })
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::astd_read(r).await?;

            // list_from: u32
            let list_from = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                list_from,
            })
        })
    }

}

