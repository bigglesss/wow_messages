use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::BattlefieldPortAction;
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm#L8):
/// ```text
/// cmsg CMSG_BATTLEFIELD_PORT = 0x02D5 {
///     Map map;
///     BattlefieldPortAction action;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_PORT {
    pub map: Map,
    pub action: BattlefieldPortAction,
}

impl crate::Message for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u32 = 0x02d5;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // action: BattlefieldPortAction
        w.write_all(&(self.action.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // action: BattlefieldPortAction
        let action: BattlefieldPortAction = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            map,
            action,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BATTLEFIELD_PORT {}

