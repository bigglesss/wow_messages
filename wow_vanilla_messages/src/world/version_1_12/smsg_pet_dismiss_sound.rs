use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_dismiss_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_dismiss_sound.wowm#L3):
/// ```text
/// smsg SMSG_PET_DISMISS_SOUND = 0x0325 {
///     u32 sound_id;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_PET_DISMISS_SOUND {
    pub sound_id: u32,
    pub position: Vector3d,
}

impl ServerMessage for SMSG_PET_DISMISS_SOUND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x0325;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        Ok(Self {
            sound_id,
            position,
        })
    }

}

