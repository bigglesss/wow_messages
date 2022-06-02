use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::MovementInfo;
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_fall_land.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_fall_land.wowm#L3):
/// ```text
/// msg MSG_MOVE_FALL_LAND = 0x00C9 {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_FALL_LAND {
    pub info: MovementInfo,
}

impl ClientMessage for MSG_MOVE_FALL_LAND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl ServerMessage for MSG_MOVE_FALL_LAND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl MSG_MOVE_FALL_LAND {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

