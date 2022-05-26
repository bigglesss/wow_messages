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
pub struct SMSG_MOVE_KNOCK_BACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub v_cos: f32,
    pub v_sin: f32,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
}

impl ServerMessageWrite for SMSG_MOVE_KNOCK_BACK {}

impl SMSG_MOVE_KNOCK_BACK {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes())?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes())?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes())?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_MOVE_KNOCK_BACK {
    const OPCODE: u16 = 0x00ef;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // v_cos: f32
        let v_cos = crate::util::read_f32_le(r)?;
        // v_sin: f32
        let v_sin = crate::util::read_f32_le(r)?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::read_f32_le(r)?;
        // vertical_speed: f32
        let vertical_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
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

            // movement_counter: u32
            let movement_counter = crate::util::tokio_read_u32_le(r).await?;

            // v_cos: f32
            let v_cos = crate::util::tokio_read_f32_le(r).await?;
            // v_sin: f32
            let v_sin = crate::util::tokio_read_f32_le(r).await?;
            // horizontal_speed: f32
            let horizontal_speed = crate::util::tokio_read_f32_le(r).await?;
            // vertical_speed: f32
            let vertical_speed = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                guid,
                movement_counter,
                v_cos,
                v_sin,
                horizontal_speed,
                vertical_speed,
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

            // movement_counter: u32
            let movement_counter = crate::util::astd_read_u32_le(r).await?;

            // v_cos: f32
            let v_cos = crate::util::astd_read_f32_le(r).await?;
            // v_sin: f32
            let v_sin = crate::util::astd_read_f32_le(r).await?;
            // horizontal_speed: f32
            let horizontal_speed = crate::util::astd_read_f32_le(r).await?;
            // vertical_speed: f32
            let vertical_speed = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                guid,
                movement_counter,
                v_cos,
                v_sin,
                horizontal_speed,
                vertical_speed,
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

impl SMSG_MOVE_KNOCK_BACK {
    pub fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

