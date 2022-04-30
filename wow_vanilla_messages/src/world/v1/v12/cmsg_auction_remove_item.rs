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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUCTION_REMOVE_ITEM {}

impl MaximumPossibleSized for CMSG_AUCTION_REMOVE_ITEM {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: Guid
        + 4 // auction_id: u32
    }
}

