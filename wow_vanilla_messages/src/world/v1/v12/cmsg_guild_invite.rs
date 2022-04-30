use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_GUILD_INVITE {
    pub invited_player: String,
}

impl ClientMessageWrite for CMSG_GUILD_INVITE {
    const OPCODE: u32 = 0x82;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_GUILD_INVITE {
    type Error = CMSG_GUILD_INVITEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // invited_player: CString
        let invited_player = crate::util::read_c_string_to_vec(r)?;
        let invited_player = String::from_utf8(invited_player)?;

        Ok(Self {
            invited_player,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // invited_player: CString
        w.write_all(self.invited_player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_INVITE {
    fn size(&self) -> usize {
        self.invited_player.len() + 1 // invited_player: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_INVITE {
    fn maximum_possible_size() -> usize {
        256 // invited_player: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_INVITEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_INVITEError {}
impl std::fmt::Display for CMSG_GUILD_INVITEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_INVITEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_INVITEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

