use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_defense_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_defense_message.wowm#L3):
/// ```text
/// smsg SMSG_DEFENSE_MESSAGE = 0x033B {
///     Map map;
///     SizedCString message;
/// }
/// ```
pub struct SMSG_DEFENSE_MESSAGE {
    pub map: Map,
    pub message: String,
}

impl crate::Message for SMSG_DEFENSE_MESSAGE {
    const OPCODE: u32 = 0x033b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // message: SizedCString
        let message = crate::util::read_u32_le(r)?;
        let message = crate::util::read_sized_c_string_to_vec(r, message)?;
        let message = String::from_utf8(message)?;;
        Ok(Self {
            map,
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DEFENSE_MESSAGE {}

impl SMSG_DEFENSE_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + self.message.len() + 5 // message: SizedCString
    }
}

