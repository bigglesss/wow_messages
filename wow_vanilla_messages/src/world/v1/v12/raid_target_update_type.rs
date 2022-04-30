use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidTargetUpdateType {
    PARTIAL,
    FULL,
}

impl ReadableAndWritable for RaidTargetUpdateType {
    type Error = RaidTargetUpdateTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for RaidTargetUpdateType {
    type Error = RaidTargetUpdateTypeError;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

}

impl RaidTargetUpdateType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::PARTIAL => 0x0,
            Self::FULL => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::PARTIAL
    }

}

impl ConstantSized for RaidTargetUpdateType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidTargetUpdateType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for RaidTargetUpdateType {
    fn default() -> Self {
        Self::PARTIAL
    }
}

impl std::fmt::Display for RaidTargetUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PARTIAL => f.write_str("PARTIAL"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u8> for RaidTargetUpdateType {
    type Error = TryFromRaidTargetUpdateTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PARTIAL),
            1 => Ok(Self::FULL),
            _ => Err(TryFromRaidTargetUpdateTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRaidTargetUpdateTypeError {
    value: u8,
}

impl TryFromRaidTargetUpdateTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RaidTargetUpdateTypeError {
    Read(std::io::Error),
    TryFrom(TryFromRaidTargetUpdateTypeError),
}

impl std::error::Error for RaidTargetUpdateTypeError {}
impl std::fmt::Display for TryFromRaidTargetUpdateTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidTargetUpdateType': '{}'", self.value))
    }
}

impl std::fmt::Display for RaidTargetUpdateTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidTargetUpdateTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRaidTargetUpdateTypeError> for RaidTargetUpdateTypeError {
    fn from(value: TryFromRaidTargetUpdateTypeError) -> Self {
        Self::TryFrom(value)
    }
}

