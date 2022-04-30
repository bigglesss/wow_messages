use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{AuraLog, AuraLogError};
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
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub auras: Vec<AuraLog>,
}

impl ServerMessageWrite for SMSG_PERIODICAURALOG {
    const OPCODE: u16 = 0x24e;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_PERIODICAURALOG {
    type Error = SMSG_PERIODICAURALOGError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed(w)?;

        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_PERIODICAURALOG {
    fn size(&self) -> usize {
        self.target.size() // target: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

impl MaximumPossibleSized for SMSG_PERIODICAURALOG {
    fn maximum_possible_size() -> usize {
        9 // target: PackedGuid
        + 9 // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + 4294967295 * AuraLog::maximum_possible_size() // auras: AuraLog[amount_of_auras]
    }
}

#[derive(Debug)]
pub enum SMSG_PERIODICAURALOGError {
    Io(std::io::Error),
    AuraLog(AuraLogError),
}

impl std::error::Error for SMSG_PERIODICAURALOGError {}
impl std::fmt::Display for SMSG_PERIODICAURALOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::AuraLog(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PERIODICAURALOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AuraLogError> for SMSG_PERIODICAURALOGError {
    fn from(e: AuraLogError) -> Self {
        Self::AuraLog(e)
    }
}

