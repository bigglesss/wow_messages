use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
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
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: u32,
    pub emote: Emote,
    pub name_length: u32,
    pub name: String,
}

impl ServerMessageWrite for SMSG_TEXT_EMOTE {}

impl MessageBody for SMSG_TEXT_EMOTE {
    const OPCODE: u16 = 0x0105;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_TEXT_EMOTEError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
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

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // text_emote: u32
            let text_emote = crate::util::tokio_read_u32_le(r).await?;

            // emote: Emote
            let emote = Emote::tokio_read(r).await?;

            // name_length: u32
            let name_length = crate::util::tokio_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            Ok(Self {
                guid,
                text_emote,
                emote,
                name_length,
                name,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // text_emote: u32
            w.write_all(&self.text_emote.to_le_bytes()).await?;

            // emote: Emote
            self.emote.tokio_write(w).await?;

            // name_length: u32
            w.write_all(&self.name_length.to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // text_emote: u32
            let text_emote = crate::util::astd_read_u32_le(r).await?;

            // emote: Emote
            let emote = Emote::astd_read(r).await?;

            // name_length: u32
            let name_length = crate::util::astd_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            Ok(Self {
                guid,
                text_emote,
                emote,
                name_length,
                name,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // text_emote: u32
            w.write_all(&self.text_emote.to_le_bytes()).await?;

            // emote: Emote
            self.emote.astd_write(w).await?;

            // name_length: u32
            w.write_all(&self.name_length.to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_TEXT_EMOTE {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // text_emote: u32
        + 4 // emote: Emote
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString
    }
}

impl MaximumPossibleSized for SMSG_TEXT_EMOTE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // text_emote: u32
        + 4 // emote: Emote
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

