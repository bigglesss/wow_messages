use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_EMOTE {
    pub emote: Emote,
    pub guid: Guid,
}

impl SMSG_EMOTE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_EMOTE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0103;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = SMSG_EMOTEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

    #[cfg(feature = "tokio")]
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
            // emote: Emote
            let emote: Emote = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            Ok(Self {
                emote,
                guid,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // emote: Emote
            let emote: Emote = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            Ok(Self {
                emote,
                guid,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_EMOTEError {
    Io(std::io::Error),
    Emote(EmoteError),
}

impl std::error::Error for SMSG_EMOTEError {}
impl std::fmt::Display for SMSG_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EmoteError> for SMSG_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

