use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MovementInfo;
use crate::world::vanilla::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_water_walk_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_water_walk_ack.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_WATER_WALK_ACK = 0x02D0 {
///     Guid guid;
///     u32 movement_counter;
///     MovementInfo info;
///     u32 apply;
/// }
/// ```
pub struct CMSG_MOVE_WATER_WALK_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
    pub apply: u32,
}

impl crate::Message for CMSG_MOVE_WATER_WALK_ACK {
    const OPCODE: u32 = 0x02d0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // apply: u32
        w.write_all(&self.apply.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // apply: u32
        let apply = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
            apply,
        })
    }

}
impl ClientMessage for CMSG_MOVE_WATER_WALK_ACK {}

impl CMSG_MOVE_WATER_WALK_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // apply: u32
    }
}

