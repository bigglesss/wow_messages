use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Os {
    WINDOWS,
    OSX,
    OTHER(u32),
}

impl ReadableAndWritable for Os {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.into())
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for Os {
    type Error = std::io::Error;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u32_le(r).await?;

        Ok(a.into())
    }

}

impl Os {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::WINDOWS => 0x57696e,
            Self::OSX => 0x4f5358,
            Self::OTHER(v) => *v,
        }
    }

    pub const fn new() -> Self {
        Self::WINDOWS
    }

}

impl ConstantSized for Os {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Os {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Os {
    fn default() -> Self {
        Self::WINDOWS
    }
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WINDOWS => f.write_str("WINDOWS"),
            Self::OSX => f.write_str("OSX"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Os {
    fn from(value: u32) -> Self {
        match value {
            5728622 => Self::WINDOWS,
            5198680 => Self::OSX,
            v => Self::OTHER(v)
        }
    }
}

