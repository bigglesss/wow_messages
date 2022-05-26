use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_RANDOM_ROLL_Server {
    pub minimum: u32,
    pub maximum: u32,
    pub actual_roll: u32,
    pub guid: Guid,
}

impl MSG_RANDOM_ROLL_Server {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 20], std::io::Error> {
        let mut array_w = [0u8; 20];
        let mut w = array_w.as_mut_slice();
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for MSG_RANDOM_ROLL_Server {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(20);
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x01fb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        // actual_roll: u32
        let actual_roll = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            minimum,
            maximum,
            actual_roll,
            guid,
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
            // minimum: u32
            let minimum = crate::util::tokio_read_u32_le(r).await?;

            // maximum: u32
            let maximum = crate::util::tokio_read_u32_le(r).await?;

            // actual_roll: u32
            let actual_roll = crate::util::tokio_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            Ok(Self {
                minimum,
                maximum,
                actual_roll,
                guid,
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
            // minimum: u32
            let minimum = crate::util::astd_read_u32_le(r).await?;

            // maximum: u32
            let maximum = crate::util::astd_read_u32_le(r).await?;

            // actual_roll: u32
            let actual_roll = crate::util::astd_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            Ok(Self {
                minimum,
                maximum,
                actual_roll,
                guid,
            })
        })
    }

}

