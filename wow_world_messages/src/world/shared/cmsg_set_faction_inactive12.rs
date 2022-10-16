use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm#L3):
/// ```text
/// cmsg CMSG_SET_FACTION_INACTIVE = 0x0317 {
///     u32 reputation_list_id;
///     Bool inactive;
/// }
/// ```
pub struct CMSG_SET_FACTION_INACTIVE {
    pub reputation_list_id: u32,
    pub inactive: bool,
}

impl crate::Message for CMSG_SET_FACTION_INACTIVE {
    const OPCODE: u32 = 0x0317;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // inactive: Bool
        w.write_all(u8::from(self.inactive).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // inactive: Bool
        let inactive = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            reputation_list_id,
            inactive,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_FACTION_INACTIVE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SET_FACTION_INACTIVE {}
