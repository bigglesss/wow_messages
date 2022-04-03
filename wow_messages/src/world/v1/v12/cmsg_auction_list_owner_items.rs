use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new3.wowm:72`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new3.wowm#L72):
/// ```text
/// cmsg CMSG_AUCTION_LIST_OWNER_ITEMS = 0x259 {
///     u64 auctioneer_guid;
///     u32 list_from;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub auctioneer_guid: u64,
    pub list_from: u32,
}

impl WorldClientMessageWrite for CMSG_AUCTION_LIST_OWNER_ITEMS {
    const OPCODE: u32 = 0x259;

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
impl WorldMessageBody for CMSG_AUCTION_LIST_OWNER_ITEMS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: u64
        let auctioneer_guid = crate::util::read_u64_le(r)?;

        // list_from: u32
        let list_from = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_from,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: u64
        w.write_all(&self.auctioneer_guid.to_le_bytes())?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUCTION_LIST_OWNER_ITEMS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_LIST_OWNER_ITEMS {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: u64
        + 4 // list_from: u32
    }
}

