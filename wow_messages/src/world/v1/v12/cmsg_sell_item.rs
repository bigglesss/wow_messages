use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_sell_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_sell_item.wowm#L3):
/// ```text
/// cmsg CMSG_SELL_ITEM = 0x1A0 {
///     u64 vendor_guid;
///     u64 item_guid;
///     u8 amount;
/// }
/// ```
pub struct CMSG_SELL_ITEM {
    pub vendor_guid: u64,
    pub item_guid: u64,
    pub amount: u8,
}

impl WorldClientMessageWrite for CMSG_SELL_ITEM {
    const OPCODE: u32 = 0x1a0;

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
impl WorldMessageBody for CMSG_SELL_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: u64
        let vendor_guid = crate::util::read_u64_le(r)?;

        // item_guid: u64
        let item_guid = crate::util::read_u64_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_guid,
            amount,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor_guid: u64
        w.write_all(&self.vendor_guid.to_le_bytes())?;

        // item_guid: u64
        w.write_all(&self.item_guid.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SELL_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SELL_ITEM {
    fn maximum_possible_size() -> usize {
        8 // vendor_guid: u64
        + 8 // item_guid: u64
        + 1 // amount: u8
    }
}

