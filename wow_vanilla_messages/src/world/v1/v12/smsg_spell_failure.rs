use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellCastResult, SpellCastResultError};
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
#[derive(Copy)]
pub struct SMSG_SPELL_FAILURE {
    pub guid: Guid,
    pub id: u32,
    pub result: SpellCastResult,
}

impl ServerMessageWrite for SMSG_SPELL_FAILURE {}

impl MessageBody for SMSG_SPELL_FAILURE {
    const OPCODE: u16 = 0x0133;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SPELL_FAILUREError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result = SpellCastResult::read(r)?;

        Ok(Self {
            guid,
            id,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELL_FAILURE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SPELL_FAILURE {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // id: u32
        + SpellCastResult::size() // result: SpellCastResult
    }
}

#[derive(Debug)]
pub enum SMSG_SPELL_FAILUREError {
    Io(std::io::Error),
    SpellCastResult(SpellCastResultError),
}

impl std::error::Error for SMSG_SPELL_FAILUREError {}
impl std::fmt::Display for SMSG_SPELL_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELL_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastResultError> for SMSG_SPELL_FAILUREError {
    fn from(e: SpellCastResultError) -> Self {
        Self::SpellCastResult(e)
    }
}

