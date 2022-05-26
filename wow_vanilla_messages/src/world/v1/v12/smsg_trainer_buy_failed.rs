use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::TrainingFailureReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRAINER_BUY_FAILED {
    pub guid: Guid,
    pub id: u32,
    pub error: TrainingFailureReason,
}

impl SMSG_TRAINER_BUY_FAILED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRAINER_BUY_FAILED {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01b4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // error: TrainingFailureReason
        let error: TrainingFailureReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            id,
            error,
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

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // error: TrainingFailureReason
            let error: TrainingFailureReason = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                error,
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

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // error: TrainingFailureReason
            let error: TrainingFailureReason = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                error,
            })
        })
    }

}

