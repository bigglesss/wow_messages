use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_swim_back_speed_change_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_swim_back_speed_change_ack.wowm#L3):
/// ```text
/// cmsg CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK = 0x02DD {
///     Guid guid;
///     u32 counter;
///     MovementInfo movement_info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
    pub new_speed: f32,
}

impl ClientMessage for CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write_into_vec(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02dd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            counter,
            movement_info,
            new_speed,
        })
    }

}

impl CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

