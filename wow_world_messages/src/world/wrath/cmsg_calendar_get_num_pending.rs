use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm#L3):
/// ```text
/// msg CMSG_CALENDAR_GET_NUM_PENDING = 0x0447 {
/// }
/// ```
pub struct CMSG_CALENDAR_GET_NUM_PENDING {
}

impl crate::Message for CMSG_CALENDAR_GET_NUM_PENDING {
    const OPCODE: u32 = 0x0447;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0447, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

