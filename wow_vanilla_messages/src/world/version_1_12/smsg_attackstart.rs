use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstart.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstart.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSTART = 0x0143 {
///     Guid attacker_guid;
///     Guid victim_guid;
/// }
/// ```
pub struct SMSG_ATTACKSTART {
    pub attacker_guid: Guid,
    pub victim_guid: Guid,
}

impl ServerMessage for SMSG_ATTACKSTART {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // attacker_guid: Guid
        w.write_all(&self.attacker_guid.guid().to_le_bytes())?;

        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0143;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // attacker_guid: Guid
        let attacker_guid = Guid::read(r)?;

        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

}
