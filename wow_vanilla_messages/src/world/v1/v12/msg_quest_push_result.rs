use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{QuestPartyMessage, QuestPartyMessageError};
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl ClientMessageWrite for MSG_QUEST_PUSH_RESULT {}

impl ServerMessageWrite for MSG_QUEST_PUSH_RESULT {}

impl MSG_QUEST_PUSH_RESULT {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&(self.message.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u16 = 0x0276;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = MSG_QUEST_PUSH_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // message: QuestPartyMessage
        let message: QuestPartyMessage = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            message,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // message: QuestPartyMessage
            let message: QuestPartyMessage = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                message,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // message: QuestPartyMessage
            let message: QuestPartyMessage = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                message,
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

#[derive(Debug)]
pub enum MSG_QUEST_PUSH_RESULTError {
    Io(std::io::Error),
    QuestPartyMessage(QuestPartyMessageError),
}

impl std::error::Error for MSG_QUEST_PUSH_RESULTError {}
impl std::fmt::Display for MSG_QUEST_PUSH_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestPartyMessage(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_QUEST_PUSH_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestPartyMessageError> for MSG_QUEST_PUSH_RESULTError {
    fn from(e: QuestPartyMessageError) -> Self {
        Self::QuestPartyMessage(e)
    }
}

