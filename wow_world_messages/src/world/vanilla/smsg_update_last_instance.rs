use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_LAST_INSTANCE = 0x0320 {
///     Map map;
/// }
/// ```
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl crate::Message for SMSG_UPDATE_LAST_INSTANCE {
    const OPCODE: u32 = 0x0320;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_UPDATE_LAST_INSTANCE {}

