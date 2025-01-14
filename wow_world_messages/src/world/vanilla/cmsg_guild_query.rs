use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_guild_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_guild_query.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_QUERY = 0x0054 {
///     u32 guild_id;
/// }
/// ```
pub struct CMSG_GUILD_QUERY {
    pub guild_id: u32,
}

impl crate::Message for CMSG_GUILD_QUERY {
    const OPCODE: u32 = 0x0054;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guild_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GUILD_QUERY {}

