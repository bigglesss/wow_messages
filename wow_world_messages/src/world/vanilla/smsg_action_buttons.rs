use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L3):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x0129 {
///     u32[120] data;
/// }
/// ```
pub struct SMSG_ACTION_BUTTONS {
    pub data: [u32; 120],
}

impl crate::Message for SMSG_ACTION_BUTTONS {
    const OPCODE: u32 = 0x0129;

    fn size_without_header(&self) -> u32 {
        480
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // data: u32[120]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 480 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // data: u32[120]
        let mut data = [u32::default(); 120];
        for i in data.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            data,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ACTION_BUTTONS {}

