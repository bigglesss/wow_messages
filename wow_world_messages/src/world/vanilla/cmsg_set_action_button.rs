use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm#L3):
/// ```text
/// cmsg CMSG_SET_ACTION_BUTTON = 0x0128 {
///     u8 button;
///     u32 action_type;
/// }
/// ```
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    /// Most significant byte determines types, rest is action.
    ///
    pub action_type: u32,
}

impl crate::Message for CMSG_SET_ACTION_BUTTON {
    const OPCODE: u32 = 0x0128;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action_type: u32
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // button: u8
        let button = crate::util::read_u8_le(r)?;

        // action_type: u32
        let action_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            button,
            action_type,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_ACTION_BUTTON {}

