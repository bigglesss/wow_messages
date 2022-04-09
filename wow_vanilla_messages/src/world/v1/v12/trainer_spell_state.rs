use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L3):
/// ```text
/// enum TrainerSpellState : u8 {
///     GREEN = 0;
///     RED = 1;
///     GRAY = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainerSpellState {
    GREEN,
    RED,
    GRAY,
}

impl ReadableAndWritable for TrainerSpellState {
    type Error = TrainerSpellStateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl TrainerSpellState {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellStateError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::GREEN => 0x0,
            Self::RED => 0x1,
            Self::GRAY => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::GREEN
    }

}

impl ConstantSized for TrainerSpellState {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TrainerSpellState {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for TrainerSpellState {
    fn default() -> Self {
        Self::GREEN
    }
}

impl std::fmt::Display for TrainerSpellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GREEN => f.write_str("GREEN"),
            Self::RED => f.write_str("RED"),
            Self::GRAY => f.write_str("GRAY"),
        }
    }
}

impl TryFrom<u8> for TrainerSpellState {
    type Error = TryFromTrainerSpellStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GREEN),
            1 => Ok(Self::RED),
            2 => Ok(Self::GRAY),
            _ => Err(TryFromTrainerSpellStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromTrainerSpellStateError {
    value: u8,
}

impl TryFromTrainerSpellStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum TrainerSpellStateError {
    Read(std::io::Error),
    TryFrom(TryFromTrainerSpellStateError),
}

impl std::error::Error for TrainerSpellStateError {}
impl std::fmt::Display for TryFromTrainerSpellStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TrainerSpellState': '{}'", self.value))
    }
}

impl std::fmt::Display for TrainerSpellStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TrainerSpellStateError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromTrainerSpellStateError> for TrainerSpellStateError {
    fn from(value: TryFromTrainerSpellStateError) -> Self {
        Self::TryFrom(value)
    }
}
