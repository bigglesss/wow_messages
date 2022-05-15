use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{FriendResult, FriendResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_FRIEND_STATUS {}

impl MessageBody for SMSG_FRIEND_STATUS {
    const OPCODE: u16 = 0x0068;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_FRIEND_STATUSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: FriendResult
        let result = FriendResult::read(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            result,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: FriendResult
        crate::util::write_u8_le(w, self.result.as_int() as u8)?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // result: FriendResult
            let result = FriendResult::tokio_read(r).await?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            Ok(Self {
                result,
                guid,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // result: FriendResult
            crate::util::tokio_write_u8_le(w, self.result.as_int() as u8).await?;

            // guid: Guid
            self.guid.tokio_write(w).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // result: FriendResult
            let result = FriendResult::astd_read(r).await?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            Ok(Self {
                result,
                guid,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // result: FriendResult
            crate::util::astd_write_u8_le(w, self.result.as_int() as u8).await?;

            // guid: Guid
            self.guid.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_FRIEND_STATUS {}

impl MaximumPossibleSized for SMSG_FRIEND_STATUS {
    fn maximum_possible_size() -> usize {
        0
        + 1 // result: FriendResult
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum SMSG_FRIEND_STATUSError {
    Io(std::io::Error),
    FriendResult(FriendResultError),
}

impl std::error::Error for SMSG_FRIEND_STATUSError {}
impl std::fmt::Display for SMSG_FRIEND_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::FriendResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_FRIEND_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FriendResultError> for SMSG_FRIEND_STATUSError {
    fn from(e: FriendResultError) -> Self {
        Self::FriendResult(e)
    }
}

