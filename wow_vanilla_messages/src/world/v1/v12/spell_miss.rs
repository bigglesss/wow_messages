use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellMissInfo, SpellMissInfoError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L27):
/// ```text
/// struct SpellMiss {
///     Guid target_guid;
///     SpellMissInfo miss_info;
/// }
/// ```
pub struct SpellMiss {
    pub target_guid: Guid,
    pub miss_info: SpellMissInfo,
}

impl ReadableAndWritable for SpellMiss {
    type Error = SpellMissError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info = SpellMissInfo::read(r)?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // miss_info: SpellMissInfo
        self.miss_info.write(w)?;

        Ok(())
    }

}

impl ConstantSized for SpellMiss {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellMiss {
    fn maximum_possible_size() -> usize {
        8 // target_guid: Guid
        + SpellMissInfo::size() // miss_info: SpellMissInfo
    }
}

#[derive(Debug)]
pub enum SpellMissError {
    Io(std::io::Error),
    SpellMissInfo(SpellMissInfoError),
}

impl std::error::Error for SpellMissError {}
impl std::fmt::Display for SpellMissError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellMissInfo(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellMissError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellMissInfoError> for SpellMissError {
    fn from(e: SpellMissInfoError) -> Self {
        Self::SpellMissInfo(e)
    }
}

