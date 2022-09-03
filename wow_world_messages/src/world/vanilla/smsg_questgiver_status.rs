use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::QuestGiverStatus;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_STATUS = 0x0183 {
///     Guid guid;
///     QuestGiverStatus status;
/// }
/// ```
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl crate::Message for SMSG_QUESTGIVER_STATUS {
    const OPCODE: u32 = 0x0183;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: QuestGiverStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // status: QuestGiverStatus
        let status: QuestGiverStatus = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            guid,
            status,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTGIVER_STATUS {}

