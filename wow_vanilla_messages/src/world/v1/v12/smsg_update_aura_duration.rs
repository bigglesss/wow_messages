use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl SMSG_UPDATE_AURA_DURATION {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_UPDATE_AURA_DURATION {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0137;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
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
            // aura_slot: u8
            let aura_slot = crate::util::tokio_read_u8_le(r).await?;

            // aura_duration: u32
            let aura_duration = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                aura_slot,
                aura_duration,
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
            // aura_slot: u8
            let aura_slot = crate::util::astd_read_u8_le(r).await?;

            // aura_duration: u32
            let aura_duration = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                aura_slot,
                aura_duration,
            })
        })
    }

}

