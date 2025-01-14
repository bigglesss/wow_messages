use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::PetTalkReason;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm#L8):
/// ```text
/// smsg SMSG_PET_ACTION_SOUND = 0x0324 {
///     Guid guid;
///     PetTalkReason reason;
/// }
/// ```
pub struct SMSG_PET_ACTION_SOUND {
    pub guid: Guid,
    pub reason: PetTalkReason,
}

impl crate::Message for SMSG_PET_ACTION_SOUND {
    const OPCODE: u32 = 0x0324;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reason: PetTalkReason
        w.write_all(&(self.reason.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // reason: PetTalkReason
        let reason: PetTalkReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            reason,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PET_ACTION_SOUND {}

