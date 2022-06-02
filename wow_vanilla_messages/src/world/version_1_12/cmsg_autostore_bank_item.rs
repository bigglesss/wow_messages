use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_BANK_ITEM = 0x0282 {
///     u8 bag_index;
///     u8 slot_index;
/// }
/// ```
pub struct CMSG_AUTOSTORE_BANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

impl ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0282;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        2
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

}
