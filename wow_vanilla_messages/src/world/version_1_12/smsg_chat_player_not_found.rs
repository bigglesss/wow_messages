use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_player_not_found.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_player_not_found.wowm#L3):
/// ```text
/// smsg SMSG_CHAT_PLAYER_NOT_FOUND = 0x02A9 {
///     CString name;
/// }
/// ```
pub struct SMSG_CHAT_PLAYER_NOT_FOUND {
    pub name: String,
}

impl ServerMessage for SMSG_CHAT_PLAYER_NOT_FOUND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x02a9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            name,
        })
    }

}

impl SMSG_CHAT_PLAYER_NOT_FOUND {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}
