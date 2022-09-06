use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::MountResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm#L3):
/// ```text
/// smsg SMSG_DISMOUNTRESULT = 0x016F {
///     MountResult result;
/// }
/// ```
pub struct SMSG_DISMOUNTRESULT {
    pub result: MountResult,
}

impl crate::Message for SMSG_DISMOUNTRESULT {
    const OPCODE: u32 = 0x016f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: MountResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: MountResult
        let result: MountResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DISMOUNTRESULT {}

