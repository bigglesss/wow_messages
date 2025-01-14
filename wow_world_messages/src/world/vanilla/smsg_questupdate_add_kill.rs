use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
///     u32 quest_id;
///     u32 create_id;
///     u32 kill_count;
///     u32 required_kill_count;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    /// Unsure of name
    ///
    pub create_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

impl crate::Message for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u32 = 0x0199;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // create_id: u32
        w.write_all(&self.create_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // create_id: u32
        let create_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            create_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

