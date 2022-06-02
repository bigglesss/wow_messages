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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_clear_trade_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_clear_trade_item.wowm#L3):
/// ```text
/// cmsg CMSG_CLEAR_TRADE_ITEM = 0x011E {
///     u8 trade_slot;
/// }
/// ```
pub struct CMSG_CLEAR_TRADE_ITEM {
    pub trade_slot: u8,
}

impl ClientMessage for CMSG_CLEAR_TRADE_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // trade_slot: u8
        w.write_all(&self.trade_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x011e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // trade_slot: u8
        let trade_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            trade_slot,
        })
    }

}

