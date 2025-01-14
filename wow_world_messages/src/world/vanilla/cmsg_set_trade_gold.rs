use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TRADE_GOLD = 0x011F {
///     u32 gold;
/// }
/// ```
pub struct CMSG_SET_TRADE_GOLD {
    pub gold: u32,
}

impl crate::Message for CMSG_SET_TRADE_GOLD {
    const OPCODE: u32 = 0x011f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // gold: u32
        w.write_all(&self.gold.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // gold: u32
        let gold = crate::util::read_u32_le(r)?;

        Ok(Self {
            gold,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_TRADE_GOLD {}

