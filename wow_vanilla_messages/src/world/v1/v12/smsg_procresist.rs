use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{LogFormat, LogFormatError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PROCRESIST {
    pub guid: Guid,
    pub target_guid: Guid,
    pub id: u32,
    pub log_format: LogFormat,
}

impl SMSG_PROCRESIST {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 21], std::io::Error> {
        let mut array_w = [0u8; 21];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // log_format: LogFormat
        w.write_all(&(self.log_format.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PROCRESIST {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(21);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // log_format: LogFormat
        w.write_all(&(self.log_format.as_int() as u8).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0260;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        21
    }

    type Error = SMSG_PROCRESISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // log_format: LogFormat
        let log_format: LogFormat = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            target_guid,
            id,
            log_format,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // log_format: LogFormat
            let log_format: LogFormat = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                target_guid,
                id,
                log_format,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // log_format: LogFormat
            let log_format: LogFormat = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                target_guid,
                id,
                log_format,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_PROCRESISTError {
    Io(std::io::Error),
    LogFormat(LogFormatError),
}

impl std::error::Error for SMSG_PROCRESISTError {}
impl std::fmt::Display for SMSG_PROCRESISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LogFormat(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PROCRESISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LogFormatError> for SMSG_PROCRESISTError {
    fn from(e: LogFormatError) -> Self {
        Self::LogFormat(e)
    }
}

