use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ServerMessageType {
    SHUTDOWN_TIME,
    RESTART_TIME,
    CUSTOM,
    SHUTDOWN_CANCELLED,
    RESTART_CANCELLED,
}

impl ReadableAndWritable for ServerMessageType {
    type Error = ServerMessageTypeError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::tokio_read_u32_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::astd_read_u32_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

}

impl ServerMessageType {
    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SHUTDOWN_TIME => 0x1,
            Self::RESTART_TIME => 0x2,
            Self::CUSTOM => 0x3,
            Self::SHUTDOWN_CANCELLED => 0x4,
            Self::RESTART_CANCELLED => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::SHUTDOWN_TIME
    }

}

impl ConstantSized for ServerMessageType {}

impl MaximumPossibleSized for ServerMessageType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for ServerMessageType {
    fn default() -> Self {
        Self::SHUTDOWN_TIME
    }
}

impl std::fmt::Display for ServerMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHUTDOWN_TIME => f.write_str("SHUTDOWN_TIME"),
            Self::RESTART_TIME => f.write_str("RESTART_TIME"),
            Self::CUSTOM => f.write_str("CUSTOM"),
            Self::SHUTDOWN_CANCELLED => f.write_str("SHUTDOWN_CANCELLED"),
            Self::RESTART_CANCELLED => f.write_str("RESTART_CANCELLED"),
        }
    }
}

impl TryFrom<u32> for ServerMessageType {
    type Error = TryFromServerMessageTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SHUTDOWN_TIME),
            2 => Ok(Self::RESTART_TIME),
            3 => Ok(Self::CUSTOM),
            4 => Ok(Self::SHUTDOWN_CANCELLED),
            5 => Ok(Self::RESTART_CANCELLED),
            _ => Err(TryFromServerMessageTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromServerMessageTypeError {
    value: u32,
}

impl TryFromServerMessageTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ServerMessageTypeError {
    Read(std::io::Error),
    TryFrom(TryFromServerMessageTypeError),
}

impl std::error::Error for ServerMessageTypeError {}
impl std::fmt::Display for TryFromServerMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ServerMessageType': '{}'", self.value))
    }
}

impl std::fmt::Display for ServerMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerMessageTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromServerMessageTypeError> for ServerMessageTypeError {
    fn from(value: TryFromServerMessageTypeError) -> Self {
        Self::TryFrom(value)
    }
}

