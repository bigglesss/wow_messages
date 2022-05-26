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
pub struct SMSG_SPELLORDAMAGE_IMMUNE {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub id: u32,
    pub unknown1: u8,
}

impl SMSG_SPELLORDAMAGE_IMMUNE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 21], std::io::Error> {
        let mut array_w = [0u8; 21];
        let mut w = array_w.as_mut_slice();
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(21);
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0263;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        21
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            caster_guid,
            target_guid,
            id,
            unknown1,
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
            // caster_guid: Guid
            let caster_guid = Guid::tokio_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                caster_guid,
                target_guid,
                id,
                unknown1,
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
            // caster_guid: Guid
            let caster_guid = Guid::astd_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // unknown1: u8
            let unknown1 = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                caster_guid,
                target_guid,
                id,
                unknown1,
            })
        })
    }

}

