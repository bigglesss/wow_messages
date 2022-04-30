use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_LEADER {
    pub new_guild_leader_name: String,
}

impl WorldClientMessageWrite for CMSG_GUILD_LEADER {
    const OPCODE: u32 = 0x90;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for CMSG_GUILD_LEADER {
    type Error = CMSG_GUILD_LEADERError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_guild_leader_name: CString
        let new_guild_leader_name = crate::util::read_c_string_to_vec(r)?;
        let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

        Ok(Self {
            new_guild_leader_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // new_guild_leader_name: CString
        w.write_all(self.new_guild_leader_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_LEADER {
    fn size(&self) -> usize {
        self.new_guild_leader_name.len() + 1 // new_guild_leader_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_LEADER {
    fn maximum_possible_size() -> usize {
        256 // new_guild_leader_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_LEADERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_LEADERError {}
impl std::fmt::Display for CMSG_GUILD_LEADERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_LEADERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_LEADERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

