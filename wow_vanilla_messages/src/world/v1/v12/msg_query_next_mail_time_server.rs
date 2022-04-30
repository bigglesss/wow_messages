use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub unread_mails: f32,
}

impl ServerMessageWrite for MSG_QUERY_NEXT_MAIL_TIME_Server {}

impl MessageBody for MSG_QUERY_NEXT_MAIL_TIME_Server {
    const OPCODE: u16 = 0x0284;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unread_mails: f32
        let unread_mails = crate::util::read_f32_le(r)?;
        Ok(Self {
            unread_mails,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unread_mails: f32
        w.write_all(&self.unread_mails.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for MSG_QUERY_NEXT_MAIL_TIME_Server {}

impl MaximumPossibleSized for MSG_QUERY_NEXT_MAIL_TIME_Server {
    fn maximum_possible_size() -> usize {
        4 // unread_mails: f32
    }
}

