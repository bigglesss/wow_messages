use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_FORCE_TURN_RATE_CHANGE {
    pub guid: Guid,
    pub move_event: u32,
    pub speed: f32,
}

impl ServerMessageWrite for SMSG_FORCE_TURN_RATE_CHANGE {}

impl SMSG_FORCE_TURN_RATE_CHANGE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(17);
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // move_event: u32
        w.write_all(&self.move_event.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_FORCE_TURN_RATE_CHANGE {
    const OPCODE: u16 = 0x02de;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // move_event: u32
        let move_event = crate::util::read_u32_le(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            move_event,
            speed,
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
            // guid: PackedGuid
            let guid = Guid::tokio_read_packed(r).await?;

            // move_event: u32
            let move_event = crate::util::tokio_read_u32_le(r).await?;

            // speed: f32
            let speed = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                guid,
                move_event,
                speed,
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
            // guid: PackedGuid
            let guid = Guid::astd_read_packed(r).await?;

            // move_event: u32
            let move_event = crate::util::astd_read_u32_le(r).await?;

            // speed: f32
            let speed = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                guid,
                move_event,
                speed,
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

impl SMSG_FORCE_TURN_RATE_CHANGE {
    pub fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
        + 4 // move_event: u32
        + 4 // speed: f32
    }
}

