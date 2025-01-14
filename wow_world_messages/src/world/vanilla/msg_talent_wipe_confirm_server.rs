use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// cmangos/vmangos/mangoszero returns guid 0 and unknown 0 when talents can not be reset
/// cmangos/vmangos/mangoszero casts spell 14876 when resetting
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_server.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_server.wowm#L5):
/// ```text
/// smsg MSG_TALENT_WIPE_CONFIRM_Server = 0x02AA {
///     Guid wiping_npc;
///     u32 cost_in_copper;
/// }
/// ```
pub struct MSG_TALENT_WIPE_CONFIRM_Server {
    pub wiping_npc: Guid,
    pub cost_in_copper: u32,
}

impl crate::Message for MSG_TALENT_WIPE_CONFIRM_Server {
    const OPCODE: u32 = 0x02aa;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::read_u32_le(r)?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_TALENT_WIPE_CONFIRM_Server {}

