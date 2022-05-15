use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MOVE_HOVER_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
    pub is_applied: u32,
}

impl ClientMessageWrite for CMSG_MOVE_HOVER_ACK {}

impl MessageBody for CMSG_MOVE_HOVER_ACK {
    const OPCODE: u16 = 0x00f6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // is_applied: u32
        let is_applied = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
            movement_info,
            is_applied,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        // is_applied: u32
        w.write_all(&self.is_applied.to_le_bytes())?;

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

            // counter: u32
            let counter = crate::util::tokio_read_u32_le(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::tokio_read(r).await?;

            // is_applied: u32
            let is_applied = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                counter,
                movement_info,
                is_applied,
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

            // counter: u32
            w.write_all(&self.counter.to_le_bytes()).await?;

            // movement_info: MovementInfo
            self.movement_info.tokio_write(w).await?;

            // is_applied: u32
            w.write_all(&self.is_applied.to_le_bytes()).await?;

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

            // counter: u32
            let counter = crate::util::astd_read_u32_le(r).await?;

            // movement_info: MovementInfo
            let movement_info = MovementInfo::astd_read(r).await?;

            // is_applied: u32
            let is_applied = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                counter,
                movement_info,
                is_applied,
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

            // counter: u32
            w.write_all(&self.counter.to_le_bytes()).await?;

            // movement_info: MovementInfo
            self.movement_info.astd_write(w).await?;

            // is_applied: u32
            w.write_all(&self.is_applied.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_MOVE_HOVER_ACK {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // is_applied: u32
    }
}

