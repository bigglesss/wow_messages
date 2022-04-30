use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellCooldownStatus;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl ServerMessageWrite for SMSG_SPELL_COOLDOWN {
    const OPCODE: u16 = 0x134;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SPELL_COOLDOWN {
    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELL_COOLDOWN {
    fn size(&self) -> usize {
        8 // guid: Guid
        + self.cooldowns.iter().fold(0, |acc, x| acc + SpellCooldownStatus::size()) // cooldowns: SpellCooldownStatus[-]
    }
}

impl MaximumPossibleSized for SMSG_SPELL_COOLDOWN {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 65536 // cooldowns: SpellCooldownStatus[-]
    }
}

