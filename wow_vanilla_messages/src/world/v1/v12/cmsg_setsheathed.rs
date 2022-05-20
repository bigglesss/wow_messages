use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SheathState, SheathStateError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl ClientMessageWrite for CMSG_SETSHEATHED {}

impl CMSG_SETSHEATHED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 1], std::io::Error> {
        let mut array_w = [0u8; 1];
        let mut w = array_w.as_mut_slice();
        // sheathed: SheathState
        w.write_all(&(self.sheathed.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for CMSG_SETSHEATHED {
    const OPCODE: u16 = 0x01e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    type Error = CMSG_SETSHEATHEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sheathed: SheathState
        let sheathed: SheathState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            sheathed,
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
            // sheathed: SheathState
            let sheathed: SheathState = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                sheathed,
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
            // sheathed: SheathState
            let sheathed: SheathState = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                sheathed,
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
pub enum CMSG_SETSHEATHEDError {
    Io(std::io::Error),
    SheathState(SheathStateError),
}

impl std::error::Error for CMSG_SETSHEATHEDError {}
impl std::fmt::Display for CMSG_SETSHEATHEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SheathState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SETSHEATHEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SheathStateError> for CMSG_SETSHEATHEDError {
    fn from(e: SheathStateError) -> Self {
        Self::SheathState(e)
    }
}

