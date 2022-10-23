use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_decline.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_decline.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_DECLINE = 0x0085 {
/// }
/// ```
pub struct CMSG_GUILD_DECLINE {
}

impl crate::Message for CMSG_GUILD_DECLINE {
    const OPCODE: u32 = 0x0085;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GUILD_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GUILD_DECLINE {}
