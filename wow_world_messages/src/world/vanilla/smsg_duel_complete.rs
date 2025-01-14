use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COMPLETE = 0x016A {
///     Bool ended_without_interruption;
/// }
/// ```
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: bool,
}

impl crate::Message for SMSG_DUEL_COMPLETE {
    const OPCODE: u32 = 0x016a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // ended_without_interruption: Bool
        w.write_all(if self.ended_without_interruption { &[1] } else { &[0] })?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // ended_without_interruption: Bool
        let ended_without_interruption = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            ended_without_interruption,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DUEL_COMPLETE {}

