use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L3):
/// ```text
/// enum BattlegroundEndStatus : u8 {
///     NOT_ENDED = 0;
///     ENDED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlegroundEndStatus {
    NOT_ENDED,
    ENDED,
}

impl ReadableAndWritable for BattlegroundEndStatus {
    type Error = BattlegroundEndStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl BattlegroundEndStatus {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundEndStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0x0,
            Self::ENDED => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::NOT_ENDED
    }

}

impl ConstantSized for BattlegroundEndStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BattlegroundEndStatus {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for BattlegroundEndStatus {
    fn default() -> Self {
        Self::NOT_ENDED
    }
}

impl std::fmt::Display for BattlegroundEndStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ENDED => f.write_str("NOT_ENDED"),
            Self::ENDED => f.write_str("ENDED"),
        }
    }
}

impl TryFrom<u8> for BattlegroundEndStatus {
    type Error = TryFromBattlegroundEndStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ENDED),
            1 => Ok(Self::ENDED),
            _ => Err(TryFromBattlegroundEndStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBattlegroundEndStatusError {
    value: u8,
}

impl TryFromBattlegroundEndStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BattlegroundEndStatusError {
    Read(std::io::Error),
    TryFrom(TryFromBattlegroundEndStatusError),
}

impl std::error::Error for BattlegroundEndStatusError {}
impl std::fmt::Display for TryFromBattlegroundEndStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BattlegroundEndStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for BattlegroundEndStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BattlegroundEndStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBattlegroundEndStatusError> for BattlegroundEndStatusError {
    fn from(value: TryFromBattlegroundEndStatusError) -> Self {
        Self::TryFrom(value)
    }
}
