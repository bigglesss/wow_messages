use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GuildEmblemResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_SAVE_GUILD_EMBLEM_Server {
    pub result: GuildEmblemResult,
}

impl ServerMessage for MSG_SAVE_GUILD_EMBLEM_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: GuildEmblemResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: GuildEmblemResult
        let result: GuildEmblemResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

