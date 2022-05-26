use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
    pub quest_title: String,
    pub guid: Guid,
}

impl ServerMessage for SMSG_QUEST_CONFIRM_ACCEPT {}

impl SMSG_QUEST_CONFIRM_ACCEPT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_title: CString
        w.write_all(self.quest_title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u16 = 0x019c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUEST_CONFIRM_ACCEPTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_title: CString
        let quest_title = crate::util::read_c_string_to_vec(r)?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // quest_title: CString
            let quest_title = crate::util::tokio_read_c_string_to_vec(r).await?;
            let quest_title = String::from_utf8(quest_title)?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            Ok(Self {
                quest_id,
                quest_title,
                guid,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // quest_title: CString
            let quest_title = crate::util::astd_read_c_string_to_vec(r).await?;
            let quest_title = String::from_utf8(quest_title)?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            Ok(Self {
                quest_id,
                quest_title,
                guid,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_QUEST_CONFIRM_ACCEPT {
    pub fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + self.quest_title.len() + 1 // quest_title: CString
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum SMSG_QUEST_CONFIRM_ACCEPTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUEST_CONFIRM_ACCEPTError {}
impl std::fmt::Display for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

