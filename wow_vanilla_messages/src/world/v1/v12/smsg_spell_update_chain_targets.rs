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
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub targets: Vec<Guid>,
}

impl SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(w)
    }
    const OPCODE: u16 = 0x0330;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: Guid
        let caster = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(r)?;

        // targets: Guid[amount_of_targets]
        let mut targets = Vec::with_capacity(amount_of_targets as usize);
        for i in 0..amount_of_targets {
            targets.push(Guid::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            targets,
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
            // caster: Guid
            let caster = Guid::tokio_read(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_targets: u32
            let amount_of_targets = crate::util::tokio_read_u32_le(r).await?;

            // targets: Guid[amount_of_targets]
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(Guid::tokio_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                targets,
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
            // caster: Guid
            let caster = Guid::astd_read(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // amount_of_targets: u32
            let amount_of_targets = crate::util::astd_read_u32_le(r).await?;

            // targets: Guid[amount_of_targets]
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(Guid::astd_read(r).await?);
            }

            Ok(Self {
                caster,
                spell,
                targets,
            })
        })
    }

}

impl SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub fn size(&self) -> usize {
        0
        + 8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + self.targets.iter().fold(0, |acc, _| acc + 8) // targets: Guid[amount_of_targets]
    }
}

