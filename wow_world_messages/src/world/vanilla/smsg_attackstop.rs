use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstop.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstop.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSTOP = 0x0144 {
///     PackedGuid player;
///     PackedGuid enemy;
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    /// cmangos/vmangos/mangoszero: set to 0 with comment: unk, can be 1 also
    ///
    pub unknown1: u32,
}

impl crate::Message for SMSG_ATTACKSTOP {
    const OPCODE: u32 = 0x0144;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // enemy: PackedGuid
        self.enemy.write_packed_guid_into_vec(w);

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // enemy: PackedGuid
        let enemy = Guid::read_packed(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ATTACKSTOP {}

impl SMSG_ATTACKSTOP {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.enemy.size() // enemy: Guid
        + 4 // unknown1: u32
    }
}

