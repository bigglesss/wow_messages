use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::ForcedReaction;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L12):
/// ```text
/// smsg SMSG_SET_FORCED_REACTIONS = 0x02A5 {
///     u32 amount_of_reactions;
///     ForcedReaction[amount_of_reactions] reactions;
/// }
/// ```
pub struct SMSG_SET_FORCED_REACTIONS {
    pub reactions: Vec<ForcedReaction>,
}

impl crate::Message for SMSG_SET_FORCED_REACTIONS {
    const OPCODE: u32 = 0x02a5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
        for i in 0..amount_of_reactions {
            reactions.push(ForcedReaction::read(r)?);
        }

        Ok(Self {
            reactions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SET_FORCED_REACTIONS {}

impl SMSG_SET_FORCED_REACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reactions: u32
        + self.reactions.len() * 8 // reactions: ForcedReaction[amount_of_reactions]
    }
}

