use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::AuraLog;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub auras: Vec<AuraLog>,
}

impl SMSG_PERIODICAURALOG {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // caster: PackedGuid
        w.write_all(&self.caster.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_PERIODICAURALOG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: PackedGuid
        w.write_all(&self.target.packed_guid())?;

        // caster: PackedGuid
        w.write_all(&self.caster.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x024e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_auras: u32
        let amount_of_auras = crate::util::read_u32_le(r)?;

        // auras: AuraLog[amount_of_auras]
        let mut auras = Vec::with_capacity(amount_of_auras as usize);
        for i in 0..amount_of_auras {
            auras.push(AuraLog::read(r)?);
        }

        Ok(Self {
            target,
            caster,
            spell,
            auras,
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
            // target: PackedGuid
            let target = Guid::tokio_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_auras: u32
            let amount_of_auras = crate::util::tokio_read_u32_le(r).await?;

            // auras: AuraLog[amount_of_auras]
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for i in 0..amount_of_auras {
                auras.push(AuraLog::tokio_read(r).await?);
            }

            Ok(Self {
                target,
                caster,
                spell,
                auras,
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
            // target: PackedGuid
            let target = Guid::astd_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_auras: u32
            let amount_of_auras = crate::util::astd_read_u32_le(r).await?;

            // auras: AuraLog[amount_of_auras]
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for i in 0..amount_of_auras {
                auras.push(AuraLog::astd_read(r).await?);
            }

            Ok(Self {
                target,
                caster,
                spell,
                auras,
            })
        })
    }

}

impl SMSG_PERIODICAURALOG {
    pub fn size(&self) -> usize {
        0
        + self.target.size() // target: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

