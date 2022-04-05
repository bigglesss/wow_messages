use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:672`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L672):
/// ```text
/// cmsg CMSG_SWAP_ITEM = 0x10C {
///     u8 destination_bag;
///     u8 destionation_slot;
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
pub struct CMSG_SWAP_ITEM {
    pub destination_bag: u8,
    pub destionation_slot: u8,
    pub source_bag: u8,
    pub source_slot: u8,
}

impl WorldClientMessageWrite for CMSG_SWAP_ITEM {
    const OPCODE: u32 = 0x10c;

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
impl WorldMessageBody for CMSG_SWAP_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(r)?;

        // destionation_slot: u8
        let destionation_slot = crate::util::read_u8_le(r)?;

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            destination_bag,
            destionation_slot,
            source_bag,
            source_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        // destionation_slot: u8
        w.write_all(&self.destionation_slot.to_le_bytes())?;

        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SWAP_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SWAP_ITEM {
    fn maximum_possible_size() -> usize {
        1 // destination_bag: u8
        + 1 // destionation_slot: u8
        + 1 // source_bag: u8
        + 1 // source_slot: u8
    }
}

