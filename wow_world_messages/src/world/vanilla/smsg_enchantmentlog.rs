use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// cmangos and vmangos/mangoszero disagree about packed and extra u8
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm#L5):
/// ```text
/// smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
///     Guid target_guid;
///     Guid caster_guid;
///     u32 item;
///     u32 spell;
///     u8 unknown1;
/// }
/// ```
pub struct SMSG_ENCHANTMENTLOG {
    pub target_guid: Guid,
    pub caster_guid: Guid,
    pub item: u32,
    pub spell: u32,
    pub unknown1: u8,
}

impl crate::Message for SMSG_ENCHANTMENTLOG {
    const OPCODE: u32 = 0x01d7;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            target_guid,
            caster_guid,
            item,
            spell,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ENCHANTMENTLOG {}

