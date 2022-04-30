use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PowerType, PowerTypeError};
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
pub struct SMSG_SPELLENERGIZELOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub spell: u32,
    pub power: PowerType,
    pub damage: u32,
}

impl ServerMessageWrite for SMSG_SPELLENERGIZELOG {
    const OPCODE: u16 = 0x151;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SPELLENERGIZELOG {
    type Error = SMSG_SPELLENERGIZELOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // power: PowerType
        let power = PowerType::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            spell,
            power,
            damage,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed(w)?;

        // caster_guid: PackedGuid
        self.caster_guid.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        self.power.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLENERGIZELOG {
    fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: PackedGuid
        + self.caster_guid.size() // caster_guid: PackedGuid
        + 4 // spell: u32
        + PowerType::size() // power: PowerType
        + 4 // damage: u32
    }
}

impl MaximumPossibleSized for SMSG_SPELLENERGIZELOG {
    fn maximum_possible_size() -> usize {
        9 // victim_guid: PackedGuid
        + 9 // caster_guid: PackedGuid
        + 4 // spell: u32
        + PowerType::maximum_possible_size() // power: PowerType
        + 4 // damage: u32
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLENERGIZELOGError {
    Io(std::io::Error),
    PowerType(PowerTypeError),
}

impl std::error::Error for SMSG_SPELLENERGIZELOGError {}
impl std::fmt::Display for SMSG_SPELLENERGIZELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PowerType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLENERGIZELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PowerTypeError> for SMSG_SPELLENERGIZELOGError {
    fn from(e: PowerTypeError) -> Self {
        Self::PowerType(e)
    }
}

