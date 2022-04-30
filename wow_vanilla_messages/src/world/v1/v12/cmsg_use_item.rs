use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    pub targets: SpellCastTargets,
}

impl WorldClientMessageWrite for CMSG_USE_ITEM {
    const OPCODE: u32 = 0xab;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_USE_ITEM {
    type Error = CMSG_USE_ITEMError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_USE_ITEM {
    fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + self.targets.size() // targets: SpellCastTargets
    }
}

impl MaximumPossibleSized for CMSG_USE_ITEM {
    fn maximum_possible_size() -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + SpellCastTargets::maximum_possible_size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum CMSG_USE_ITEMError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
}

impl std::error::Error for CMSG_USE_ITEMError {}
impl std::fmt::Display for CMSG_USE_ITEMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_USE_ITEMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for CMSG_USE_ITEMError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

