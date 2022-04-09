use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm#L3):
/// ```text
/// enum DuelWinnerReason : u8 {
///     WON = 0;
///     FLED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum DuelWinnerReason {
    WON,
    FLED,
}

impl ReadableAndWritable for DuelWinnerReason {
    type Error = DuelWinnerReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl DuelWinnerReason {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, DuelWinnerReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::WON => 0x0,
            Self::FLED => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::WON
    }

}

impl ConstantSized for DuelWinnerReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for DuelWinnerReason {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for DuelWinnerReason {
    fn default() -> Self {
        Self::WON
    }
}

impl std::fmt::Display for DuelWinnerReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WON => f.write_str("WON"),
            Self::FLED => f.write_str("FLED"),
        }
    }
}

impl TryFrom<u8> for DuelWinnerReason {
    type Error = TryFromDuelWinnerReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WON),
            1 => Ok(Self::FLED),
            _ => Err(TryFromDuelWinnerReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromDuelWinnerReasonError {
    value: u8,
}

impl TryFromDuelWinnerReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum DuelWinnerReasonError {
    Read(std::io::Error),
    TryFrom(TryFromDuelWinnerReasonError),
}

impl std::error::Error for DuelWinnerReasonError {}
impl std::fmt::Display for TryFromDuelWinnerReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'DuelWinnerReason': '{}'", self.value))
    }
}

impl std::fmt::Display for DuelWinnerReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for DuelWinnerReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromDuelWinnerReasonError> for DuelWinnerReasonError {
    fn from(value: TryFromDuelWinnerReasonError) -> Self {
        Self::TryFrom(value)
    }
}
