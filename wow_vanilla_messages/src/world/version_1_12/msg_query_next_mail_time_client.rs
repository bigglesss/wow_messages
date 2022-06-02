use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm#L3):
/// ```text
/// cmsg MSG_QUERY_NEXT_MAIL_TIME_Client = 0x0284 {
/// }
/// ```
pub struct MSG_QUERY_NEXT_MAIL_TIME_Client {
}

impl ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x0284;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        Ok(Self {
        })
    }

}

