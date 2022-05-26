use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BgTypeId, BgTypeIdError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_GROUP_JOINED_BATTLEGROUND {
    pub id: BgTypeId,
}

impl SMSG_GROUP_JOINED_BATTLEGROUND {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // id: BgTypeId
        w.write_all(&(self.id.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: BgTypeId
        w.write_all(&(self.id.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02e8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = SMSG_GROUP_JOINED_BATTLEGROUNDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: BgTypeId
        let id: BgTypeId = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            id,
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
            // id: BgTypeId
            let id: BgTypeId = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                id,
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
            // id: BgTypeId
            let id: BgTypeId = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                id,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_GROUP_JOINED_BATTLEGROUNDError {
    Io(std::io::Error),
    BgTypeId(BgTypeIdError),
}

impl std::error::Error for SMSG_GROUP_JOINED_BATTLEGROUNDError {}
impl std::fmt::Display for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BgTypeId(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BgTypeIdError> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e: BgTypeIdError) -> Self {
        Self::BgTypeId(e)
    }
}

