use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_MOVE_TELEPORT_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub time_in_msecs: u32,
}

impl MSG_MOVE_TELEPORT_ACK {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for MSG_MOVE_TELEPORT_ACK {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // time_in_msecs: u32
        let time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            time_in_msecs,
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

            // movement_counter: u32
            let movement_counter = crate::util::tokio_read_u32_le(r).await?;

            // time_in_msecs: u32
            let time_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                movement_counter,
                time_in_msecs,
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

            // movement_counter: u32
            let movement_counter = crate::util::astd_read_u32_le(r).await?;

            // time_in_msecs: u32
            let time_in_msecs = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                movement_counter,
                time_in_msecs,
            })
        })
    }

}

impl ServerMessage for MSG_MOVE_TELEPORT_ACK {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // time_in_msecs: u32
        let time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            time_in_msecs,
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

            // movement_counter: u32
            let movement_counter = crate::util::tokio_read_u32_le(r).await?;

            // time_in_msecs: u32
            let time_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                movement_counter,
                time_in_msecs,
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

            // movement_counter: u32
            let movement_counter = crate::util::astd_read_u32_le(r).await?;

            // time_in_msecs: u32
            let time_in_msecs = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                movement_counter,
                time_in_msecs,
            })
        })
    }

}

