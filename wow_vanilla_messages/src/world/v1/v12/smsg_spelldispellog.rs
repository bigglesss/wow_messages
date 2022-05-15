use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLDISPELLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessageWrite for SMSG_SPELLDISPELLOG {}

impl MessageBody for SMSG_SPELLDISPELLOG {
    const OPCODE: u16 = 0x027b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim: Guid
        let victim = Guid::read(r)?;

        // caster: Guid
        let caster = Guid::read(r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(r)?;

        // spells: u32[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            victim,
            caster,
            spells,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim: Guid
        self.victim.write(w)?;

        // caster: Guid
        self.caster.write(w)?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

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
            // victim: Guid
            let victim = Guid::tokio_read(r).await?;

            // caster: Guid
            let caster = Guid::tokio_read(r).await?;

            // amount_of_spells: u32
            let amount_of_spells = crate::util::tokio_read_u32_le(r).await?;

            // spells: u32[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(crate::util::tokio_read_u32_le(r).await?);
            }

            Ok(Self {
                victim,
                caster,
                spells,
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
            // victim: Guid
            self.victim.tokio_write(w).await?;

            // caster: Guid
            self.caster.tokio_write(w).await?;

            // amount_of_spells: u32
            w.write_all(&(self.spells.len() as u32).to_le_bytes()).await?;

            // spells: u32[amount_of_spells]
            for i in self.spells.iter() {
                w.write_all(&i.to_le_bytes()).await?;
            }

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
            // victim: Guid
            let victim = Guid::astd_read(r).await?;

            // caster: Guid
            let caster = Guid::astd_read(r).await?;

            // amount_of_spells: u32
            let amount_of_spells = crate::util::astd_read_u32_le(r).await?;

            // spells: u32[amount_of_spells]
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(crate::util::astd_read_u32_le(r).await?);
            }

            Ok(Self {
                victim,
                caster,
                spells,
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
            // victim: Guid
            self.victim.astd_write(w).await?;

            // caster: Guid
            self.caster.astd_write(w).await?;

            // amount_of_spells: u32
            w.write_all(&(self.spells.len() as u32).to_le_bytes()).await?;

            // spells: u32[amount_of_spells]
            for i in self.spells.iter() {
                w.write_all(&i.to_le_bytes()).await?;
            }

            Ok(())
        })
    }

}

impl SMSG_SPELLDISPELLOG {
    pub fn size(&self) -> usize {
        0
        + 8 // victim: Guid
        + 8 // caster: Guid
        + 4 // amount_of_spells: u32
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
    }
}

