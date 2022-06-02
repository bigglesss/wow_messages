use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_channel_set_owner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_channel_set_owner.wowm#L3):
/// ```text
/// cmsg CMSG_CHANNEL_SET_OWNER = 0x009D {
///     CString channel_name;
///     CString new_owner;
/// }
/// ```
pub struct CMSG_CHANNEL_SET_OWNER {
    pub channel_name: String,
    pub new_owner: String,
}

impl ClientMessage for CMSG_CHANNEL_SET_OWNER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // new_owner: CString
        w.write_all(self.new_owner.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x009d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // new_owner: CString
        let new_owner = crate::util::read_c_string_to_vec(r)?;
        let new_owner = String::from_utf8(new_owner)?;

        Ok(Self {
            channel_name,
            new_owner,
        })
    }

}

impl CMSG_CHANNEL_SET_OWNER {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + self.new_owner.len() + 1 // new_owner: CString
    }
}
