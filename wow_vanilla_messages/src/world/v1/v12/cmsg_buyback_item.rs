use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{BuybackSlot, BuybackSlotError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_BUYBACK_ITEM {
    pub guid: Guid,
    pub slot: BuybackSlot,
}

impl ClientMessageWrite for CMSG_BUYBACK_ITEM {}

impl MessageBody for CMSG_BUYBACK_ITEM {
    const OPCODE: u16 = 0x0290;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_BUYBACK_ITEMError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // slot: BuybackSlot
        let slot: BuybackSlot = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            slot,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // slot: BuybackSlot
        w.write_all(&(self.slot.as_int() as u32).to_le_bytes())?;

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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // slot: BuybackSlot
            let slot: BuybackSlot = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                slot,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // slot: BuybackSlot
            w.write_all(&(self.slot.as_int() as u32).to_le_bytes()).await?;

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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // slot: BuybackSlot
            let slot: BuybackSlot = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                slot,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // slot: BuybackSlot
            w.write_all(&(self.slot.as_int() as u32).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_BUYBACK_ITEM {}

impl MaximumPossibleSized for CMSG_BUYBACK_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // slot: BuybackSlot
    }
}

#[derive(Debug)]
pub enum CMSG_BUYBACK_ITEMError {
    Io(std::io::Error),
    BuybackSlot(BuybackSlotError),
}

impl std::error::Error for CMSG_BUYBACK_ITEMError {}
impl std::fmt::Display for CMSG_BUYBACK_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BuybackSlot(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BUYBACK_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BuybackSlotError> for CMSG_BUYBACK_ITEMError {
    fn from(e: BuybackSlotError) -> Self {
        Self::BuybackSlot(e)
    }
}

