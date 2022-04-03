use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new3.wowm:44`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new3.wowm#L44):
/// ```text
/// cmsg CMSG_AUCTION_SELL_ITEM = 0x256 {
///     u64 auctioneer_guid;
///     u64 object_guid;
///     u32 stack_size;
///     u32 starting_bid;
///     u32 buyout;
///     u32 auction_duration_in_minutes;
/// }
/// ```
pub struct CMSG_AUCTION_SELL_ITEM {
    pub auctioneer_guid: u64,
    pub object_guid: u64,
    pub stack_size: u32,
    pub starting_bid: u32,
    pub buyout: u32,
    pub auction_duration_in_minutes: u32,
}

impl WorldClientMessageWrite for CMSG_AUCTION_SELL_ITEM {
    const OPCODE: u32 = 0x256;

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
impl WorldMessageBody for CMSG_AUCTION_SELL_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: u64
        let auctioneer_guid = crate::util::read_u64_le(r)?;

        // object_guid: u64
        let object_guid = crate::util::read_u64_le(r)?;

        // stack_size: u32
        let stack_size = crate::util::read_u32_le(r)?;

        // starting_bid: u32
        let starting_bid = crate::util::read_u32_le(r)?;

        // buyout: u32
        let buyout = crate::util::read_u32_le(r)?;

        // auction_duration_in_minutes: u32
        let auction_duration_in_minutes = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            object_guid,
            stack_size,
            starting_bid,
            buyout,
            auction_duration_in_minutes,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: u64
        w.write_all(&self.auctioneer_guid.to_le_bytes())?;

        // object_guid: u64
        w.write_all(&self.object_guid.to_le_bytes())?;

        // stack_size: u32
        w.write_all(&self.stack_size.to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUCTION_SELL_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_SELL_ITEM {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: u64
        + 8 // object_guid: u64
        + 4 // stack_size: u32
        + 4 // starting_bid: u32
        + 4 // buyout: u32
        + 4 // auction_duration_in_minutes: u32
    }
}

