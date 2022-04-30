use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: u32,
    pub emote: Emote,
    pub name_length: u32,
    pub name: String,
}

impl ServerMessageWrite for SMSG_TEXT_EMOTE {
    const OPCODE: u16 = 0x105;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_TEXT_EMOTE {
    type Error = SMSG_TEXT_EMOTEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // text_emote: u32
        let text_emote = crate::util::read_u32_le(r)?;

        // emote: Emote
        let emote = Emote::read(r)?;

        // name_length: u32
        let name_length = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            guid,
            text_emote,
            emote,
            name_length,
            name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes())?;

        // emote: Emote
        self.emote.write(w)?;

        // name_length: u32
        w.write_all(&self.name_length.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_TEXT_EMOTE {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // text_emote: u32
        + Emote::size() // emote: Emote
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_TEXT_EMOTE {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // text_emote: u32
        + Emote::maximum_possible_size() // emote: Emote
        + 4 // name_length: u32
        + 256 // name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_TEXT_EMOTEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Emote(EmoteError),
}

impl std::error::Error for SMSG_TEXT_EMOTEError {}
impl std::fmt::Display for SMSG_TEXT_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TEXT_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_TEXT_EMOTEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<EmoteError> for SMSG_TEXT_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

