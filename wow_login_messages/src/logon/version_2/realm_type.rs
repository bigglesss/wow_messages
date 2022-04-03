use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L2):
/// ```text
/// enum RealmType : u32 {
///     PLAYER_VS_ENVIRONMENT = 0;
///     PLAYER_VS_PLAYER = 1;
///     ROLEPLAYING = 6;
///     ROLEPLAYING_PLAYER_VS_PLAYER = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmType {
    PLAYER_VS_ENVIRONMENT,
    PLAYER_VS_PLAYER,
    ROLEPLAYING,
    ROLEPLAYING_PLAYER_VS_PLAYER,
}

impl ReadableAndWritable for RealmType {
    type Error = RealmTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl RealmType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => 0x0,
            Self::PLAYER_VS_PLAYER => 0x1,
            Self::ROLEPLAYING => 0x6,
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => 0x8,
        }
    }

    pub const fn new() -> Self {
        Self::PLAYER_VS_ENVIRONMENT
    }

}

impl ConstantSized for RealmType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RealmType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for RealmType {
    fn default() -> Self {
        Self::PLAYER_VS_ENVIRONMENT
    }
}

impl std::fmt::Display for RealmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => f.write_str("PLAYER_VS_ENVIRONMENT"),
            Self::PLAYER_VS_PLAYER => f.write_str("PLAYER_VS_PLAYER"),
            Self::ROLEPLAYING => f.write_str("ROLEPLAYING"),
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => f.write_str("ROLEPLAYING_PLAYER_VS_PLAYER"),
        }
    }
}

impl TryFrom<u32> for RealmType {
    type Error = TryFromRealmTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PLAYER_VS_ENVIRONMENT),
            1 => Ok(Self::PLAYER_VS_PLAYER),
            6 => Ok(Self::ROLEPLAYING),
            8 => Ok(Self::ROLEPLAYING_PLAYER_VS_PLAYER),
            _ => Err(TryFromRealmTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRealmTypeError {
    value: u32,
}

impl TryFromRealmTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RealmTypeError {
    Read(std::io::Error),
    TryFrom(TryFromRealmTypeError),
}

impl std::error::Error for RealmTypeError {}
impl std::fmt::Display for TryFromRealmTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RealmType': '{}'", self.value))
    }
}

impl std::fmt::Display for RealmTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RealmTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRealmTypeError> for RealmTypeError {
    fn from(value: TryFromRealmTypeError) -> Self {
        Self::TryFrom(value)
    }
}

