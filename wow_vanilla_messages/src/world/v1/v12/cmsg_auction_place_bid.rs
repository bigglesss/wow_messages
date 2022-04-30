use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_AUCTION_PLACE_BID {
    pub auctioneer_guid: Guid,
    pub auction_id: u32,
    pub price: u32,
}

impl WorldClientMessageWrite for CMSG_AUCTION_PLACE_BID {
    const OPCODE: u32 = 0x25a;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_AUCTION_PLACE_BID {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        // price: u32
        let price = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            auction_id,
            price,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // price: u32
        w.write_all(&self.price.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUCTION_PLACE_BID {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_PLACE_BID {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: Guid
        + 4 // auction_id: u32
        + 4 // price: u32
    }
}

