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
pub struct SMSG_SUPERCEDED_SPELL {
    pub new_spell_id: u16,
    pub old_spell_id: u16,
}

impl SMSG_SUPERCEDED_SPELL {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_SUPERCEDED_SPELL {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x012c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_spell_id: u16
        let new_spell_id = crate::util::read_u16_le(r)?;

        // old_spell_id: u16
        let old_spell_id = crate::util::read_u16_le(r)?;

        Ok(Self {
            new_spell_id,
            old_spell_id,
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
            // new_spell_id: u16
            let new_spell_id = crate::util::tokio_read_u16_le(r).await?;

            // old_spell_id: u16
            let old_spell_id = crate::util::tokio_read_u16_le(r).await?;

            Ok(Self {
                new_spell_id,
                old_spell_id,
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
            // new_spell_id: u16
            let new_spell_id = crate::util::astd_read_u16_le(r).await?;

            // old_spell_id: u16
            let old_spell_id = crate::util::astd_read_u16_le(r).await?;

            Ok(Self {
                new_spell_id,
                old_spell_id,
            })
        })
    }

}

