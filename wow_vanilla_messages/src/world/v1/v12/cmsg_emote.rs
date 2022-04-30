use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Emote, EmoteError};
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
#[derive(Copy)]
pub struct CMSG_EMOTE {
    pub emote: Emote,
}

impl ClientMessageWrite for CMSG_EMOTE {}

impl MessageBody for CMSG_EMOTE {
    const OPCODE: u16 = 0x0102;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_EMOTEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote = Emote::read(r)?;

        Ok(Self {
            emote,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: Emote
        self.emote.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_EMOTE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_EMOTE {
    fn maximum_possible_size() -> usize {
        Emote::size() // emote: Emote
    }
}

#[derive(Debug)]
pub enum CMSG_EMOTEError {
    Io(std::io::Error),
    Emote(EmoteError),
}

impl std::error::Error for CMSG_EMOTEError {}
impl std::fmt::Display for CMSG_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EmoteError> for CMSG_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

