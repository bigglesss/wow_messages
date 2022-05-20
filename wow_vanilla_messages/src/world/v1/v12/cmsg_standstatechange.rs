use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{UnitStandState, UnitStandStateError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl ClientMessageWrite for CMSG_STANDSTATECHANGE {}

impl MessageBody for CMSG_STANDSTATECHANGE {
    const OPCODE: u16 = 0x0101;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_STANDSTATECHANGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // animation_state: UnitStandState
        let animation_state: UnitStandState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // animation_state: UnitStandState
        w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes())?;

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
            // animation_state: UnitStandState
            let animation_state: UnitStandState = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                animation_state,
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
            // animation_state: UnitStandState
            w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes()).await?;

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
            // animation_state: UnitStandState
            let animation_state: UnitStandState = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                animation_state,
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
            // animation_state: UnitStandState
            w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_STANDSTATECHANGE {
    pub(crate) fn size() -> usize {
        0
        + 1 // animation_state: UnitStandState
    }
}

#[derive(Debug)]
pub enum CMSG_STANDSTATECHANGEError {
    Io(std::io::Error),
    UnitStandState(UnitStandStateError),
}

impl std::error::Error for CMSG_STANDSTATECHANGEError {}
impl std::fmt::Display for CMSG_STANDSTATECHANGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnitStandState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_STANDSTATECHANGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<UnitStandStateError> for CMSG_STANDSTATECHANGEError {
    fn from(e: UnitStandStateError) -> Self {
        Self::UnitStandState(e)
    }
}

