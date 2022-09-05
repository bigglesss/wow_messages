use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_client.wowm#L3):
/// ```text
/// cmsg MSG_TALENT_WIPE_CONFIRM_Client = 0x02AA {
///     Guid wiping_npc;
/// }
/// ```
pub struct MSG_TALENT_WIPE_CONFIRM_Client {
    pub wiping_npc: Guid,
}

impl crate::Message for MSG_TALENT_WIPE_CONFIRM_Client {
    const OPCODE: u32 = 0x02aa;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        Ok(Self {
            wiping_npc,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_TALENT_WIPE_CONFIRM_Client {}

