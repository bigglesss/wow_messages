use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellCooldownStatus;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl ServerMessageWrite for SMSG_SPELL_COOLDOWN {}

impl MessageBody for SMSG_SPELL_COOLDOWN {
    const OPCODE: u16 = 0x0134;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // cooldowns: SpellCooldownStatus[-]
        let mut current_size = {
            8 // guid: Guid
        };
        let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            cooldowns.push(SpellCooldownStatus::read(r)?);
            current_size += 1;
        }

        Ok(Self {
            guid,
            cooldowns,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write(w)?;
        }

        Ok(())
    }

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

            // cooldowns: SpellCooldownStatus[-]
            let mut current_size = {
                8 // guid: Guid
            };
            let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                cooldowns.push(SpellCooldownStatus::tokio_read(r).await?);
                current_size += 1;
            }

            Ok(Self {
                guid,
                cooldowns,
            })
        })
    }

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

            // cooldowns: SpellCooldownStatus[-]
            for i in self.cooldowns.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

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

            // cooldowns: SpellCooldownStatus[-]
            let mut current_size = {
                8 // guid: Guid
            };
            let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                cooldowns.push(SpellCooldownStatus::astd_read(r).await?);
                current_size += 1;
            }

            Ok(Self {
                guid,
                cooldowns,
            })
        })
    }

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

            // cooldowns: SpellCooldownStatus[-]
            for i in self.cooldowns.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_SPELL_COOLDOWN {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + self.cooldowns.iter().fold(0, |acc, x| acc + SpellCooldownStatus::size()) // cooldowns: SpellCooldownStatus[-]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_COOLDOWN {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

