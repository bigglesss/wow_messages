use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_autostore_loot_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_LOOT_ITEM = 0x0108 {
///     u8 item_slot;
/// }
/// ```
pub struct CMSG_AUTOSTORE_LOOT_ITEM {
    pub item_slot: u8,
}

impl crate::Message for CMSG_AUTOSTORE_LOOT_ITEM {
    const OPCODE: u32 = 0x0108;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AUTOSTORE_LOOT_ITEM {}

