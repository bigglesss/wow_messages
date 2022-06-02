use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_ENCHANT_TIME_UPDATE = 0x01EB {
///     Guid item_guid;
///     u32 slot;
///     u32 duration;
///     Guid player_guid;
/// }
/// ```
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item_guid: Guid,
    /// # Comment
    ///
    /// Possibly used with EnchantmentSlot enum.
    pub slot: u32,
    pub duration: u32,
    pub player_guid: Guid,
}

impl ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player_guid: Guid
        w.write_all(&self.player_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01eb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // player_guid: Guid
        let player_guid = Guid::read(r)?;

        Ok(Self {
            item_guid,
            slot,
            duration,
            player_guid,
        })
    }

}

