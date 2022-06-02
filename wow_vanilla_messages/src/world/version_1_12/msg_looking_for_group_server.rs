use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm#L3):
/// ```text
/// smsg MSG_LOOKING_FOR_GROUP_Server = 0x01FF {
///     u32 unknown1;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Server {
    /// # Comment
    ///
    /// vmangos sets to 0. cmangos/mangoszero don't implement
    pub unknown1: u32,
}

impl ServerMessage for MSG_LOOKING_FOR_GROUP_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ff;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
        })
    }

}

