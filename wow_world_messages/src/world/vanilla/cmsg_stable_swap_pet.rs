use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_stable_swap_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_stable_swap_pet.wowm#L3):
/// ```text
/// cmsg CMSG_STABLE_SWAP_PET = 0x0275 {
///     Guid npc;
///     u32 pet_slot;
/// }
/// ```
pub struct CMSG_STABLE_SWAP_PET {
    pub npc: Guid,
    pub pet_slot: u32,
}

impl crate::Message for CMSG_STABLE_SWAP_PET {
    const OPCODE: u32 = 0x0275;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // pet_slot: u32
        w.write_all(&self.pet_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        // pet_slot: u32
        let pet_slot = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            pet_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_STABLE_SWAP_PET {}

