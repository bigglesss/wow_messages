use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_normal_fall.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_normal_fall.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_NORMAL_FALL = 0x0306 {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_NORMAL_FALL {
    pub guid: Guid,
}

impl ServerMessage for SMSG_SPLINE_MOVE_NORMAL_FALL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0306;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}

impl SMSG_SPLINE_MOVE_NORMAL_FALL {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

