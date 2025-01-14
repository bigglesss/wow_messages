use std::convert::{TryFrom, TryInto};
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_chng_transport.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_chng_transport.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_CHNG_TRANSPORT = 0x038D {
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_CHNG_TRANSPORT {
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_CHNG_TRANSPORT {
    const OPCODE: u32 = 0x038d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_MOVE_CHNG_TRANSPORT {}

impl CMSG_MOVE_CHNG_TRANSPORT {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

