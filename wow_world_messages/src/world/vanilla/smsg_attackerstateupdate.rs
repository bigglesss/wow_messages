use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
///     u32 hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
/// }
/// ```
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: u32,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
}

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // attacker: PackedGuid
        self.attacker.write_packed_guid_into_vec(w);

        // target: PackedGuid
        self.target.write_packed_guid_into_vec(w);

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // hit_info: u32
        let hit_info = crate::util::read_u32_le(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // hit_info: u32
        + self.attacker.size() // attacker: Guid
        + self.target.size() // target: Guid
        + 4 // total_damage: u32
    }
}

