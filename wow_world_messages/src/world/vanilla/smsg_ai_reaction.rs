use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::AiReaction;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm#L21):
/// ```text
/// smsg SMSG_AI_REACTION = 0x013C {
///     Guid guid;
///     AiReaction reaction;
/// }
/// ```
pub struct SMSG_AI_REACTION {
    pub guid: Guid,
    pub reaction: AiReaction,
}

impl crate::Message for SMSG_AI_REACTION {
    const OPCODE: u32 = 0x013c;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reaction: AiReaction
        w.write_all(&(self.reaction.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // reaction: AiReaction
        let reaction: AiReaction = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            reaction,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_AI_REACTION {}

