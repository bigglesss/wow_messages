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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_get_mail_list.wowm#L3):
/// ```text
/// cmsg CMSG_GET_MAIL_LIST = 0x023A {
///     Guid mailbox_guid;
/// }
/// ```
pub struct CMSG_GET_MAIL_LIST {
    pub mailbox_guid: Guid,
}

impl ClientMessage for CMSG_GET_MAIL_LIST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // mailbox_guid: Guid
        w.write_all(&self.mailbox_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x023a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::read(r)?;

        Ok(Self {
            mailbox_guid,
        })
    }

}

