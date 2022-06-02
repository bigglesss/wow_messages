use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_all_passed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_all_passed.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_ALL_PASSED = 0x029E {
///     Guid looted_target_guid;
///     u32 loot_slot;
///     u32 item_id;
///     u32 item_random_property_id;
///     u32 item_random_suffix_id;
/// }
/// ```
pub struct SMSG_LOOT_ALL_PASSED {
    pub looted_target_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_property_id: u32,
    pub item_random_suffix_id: u32,
}

impl ServerMessage for SMSG_LOOT_ALL_PASSED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x029e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_random_suffix_id: u32
        let item_random_suffix_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            looted_target_guid,
            loot_slot,
            item_id,
            item_random_property_id,
            item_random_suffix_id,
        })
    }

}

