use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SellItemResult, SellItemResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SELL_ITEM {
    pub guid: Guid,
    pub item: Guid,
    pub result: SellItemResult,
}

impl ServerMessageWrite for SMSG_SELL_ITEM {}

impl SMSG_SELL_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 17], std::io::Error> {
        let mut array_w = [0u8; 17];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: SellItemResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_SELL_ITEM {
    const OPCODE: u16 = 0x01a1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SELL_ITEMError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // result: SellItemResult
        let result: SellItemResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            item,
            result,
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

            // item: Guid
            let item = Guid::tokio_read(r).await?;

            // result: SellItemResult
            let result: SellItemResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                item,
                result,
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

            // item: Guid
            let item = Guid::astd_read(r).await?;

            // result: SellItemResult
            let result: SellItemResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                item,
                result,
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

impl SMSG_SELL_ITEM {
    pub(crate) fn size() -> usize {
        0
        + 8 // guid: Guid
        + 8 // item: Guid
        + 1 // result: SellItemResult
    }
}

#[derive(Debug)]
pub enum SMSG_SELL_ITEMError {
    Io(std::io::Error),
    SellItemResult(SellItemResultError),
}

impl std::error::Error for SMSG_SELL_ITEMError {}
impl std::fmt::Display for SMSG_SELL_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SellItemResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SELL_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SellItemResultError> for SMSG_SELL_ITEMError {
    fn from(e: SellItemResultError) -> Self {
        Self::SellItemResult(e)
    }
}

