use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_npc_text_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_npc_text_query.wowm#L3):
/// ```text
/// cmsg CMSG_NPC_TEXT_QUERY = 0x017F {
///     u32 text_id;
///     Guid guid;
/// }
/// ```
pub struct CMSG_NPC_TEXT_QUERY {
    pub text_id: u32,
    pub guid: Guid,
}

impl ClientMessage for CMSG_NPC_TEXT_QUERY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x017f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            text_id,
            guid,
        })
    }

}
