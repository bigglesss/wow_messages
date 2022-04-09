use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L3):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x1A2 {
///     u64 vendor_guid;
///     u32 item_id;
///     u8 amount;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_BUY_ITEM {
    pub vendor_guid: u64,
    pub item_id: u32,
    pub amount: u8,
    pub unknown1: u8,
}

impl WorldClientMessageWrite for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x1a2;

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
impl WorldMessageBody for CMSG_BUY_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: u64
        let vendor_guid = crate::util::read_u64_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_id,
            amount,
            unknown1,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor_guid: u64
        w.write_all(&self.vendor_guid.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_BUY_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_BUY_ITEM {
    fn maximum_possible_size() -> usize {
        8 // vendor_guid: u64
        + 4 // item_id: u32
        + 1 // amount: u8
        + 1 // unknown1: u8
    }
}
