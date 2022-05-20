use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RollVote, RollVoteError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_ROLL {
    pub item_guid: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl ClientMessageWrite for CMSG_LOOT_ROLL {}

impl MessageBody for CMSG_LOOT_ROLL {
    const OPCODE: u16 = 0x02a0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_LOOT_ROLLError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            item_guid,
            item_slot,
            vote,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
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
            // item_guid: Guid
            let item_guid = Guid::tokio_read(r).await?;

            // item_slot: u32
            let item_slot = crate::util::tokio_read_u32_le(r).await?;

            // vote: RollVote
            let vote: RollVote = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                item_guid,
                item_slot,
                vote,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item_guid: Guid
            w.write_all(&self.item_guid.guid().to_le_bytes()).await?;

            // item_slot: u32
            w.write_all(&self.item_slot.to_le_bytes()).await?;

            // vote: RollVote
            w.write_all(&(self.vote.as_int() as u8).to_le_bytes()).await?;

            Ok(())
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
            // item_guid: Guid
            let item_guid = Guid::astd_read(r).await?;

            // item_slot: u32
            let item_slot = crate::util::astd_read_u32_le(r).await?;

            // vote: RollVote
            let vote: RollVote = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                item_guid,
                item_slot,
                vote,
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
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item_guid: Guid
            w.write_all(&self.item_guid.guid().to_le_bytes()).await?;

            // item_slot: u32
            w.write_all(&self.item_slot.to_le_bytes()).await?;

            // vote: RollVote
            w.write_all(&(self.vote.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_LOOT_ROLL {
    pub(crate) fn size() -> usize {
        0
        + 8 // item_guid: Guid
        + 4 // item_slot: u32
        + 1 // vote: RollVote
    }
}

#[derive(Debug)]
pub enum CMSG_LOOT_ROLLError {
    Io(std::io::Error),
    RollVote(RollVoteError),
}

impl std::error::Error for CMSG_LOOT_ROLLError {}
impl std::fmt::Display for CMSG_LOOT_ROLLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RollVote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_LOOT_ROLLError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RollVoteError> for CMSG_LOOT_ROLLError {
    fn from(e: RollVoteError) -> Self {
        Self::RollVote(e)
    }
}

