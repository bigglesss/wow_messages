use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{TimerType, TimerTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PAUSE_MIRROR_TIMER {
    pub timer: TimerType,
    pub is_frozen: u8,
}

impl ServerMessageWrite for SMSG_PAUSE_MIRROR_TIMER {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PAUSE_MIRROR_TIMER {
    const OPCODE: u16 = 0x01da;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PAUSE_MIRROR_TIMERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer = TimerType::read(r)?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // timer: TimerType
        self.timer.write(w)?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer = TimerType::tokio_read(r).await?;

        // is_frozen: u8
        let is_frozen = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // timer: TimerType
        self.timer.tokio_write(w).await?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer = TimerType::astd_read(r).await?;

        // is_frozen: u8
        let is_frozen = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // timer: TimerType
        self.timer.astd_write(w).await?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_PAUSE_MIRROR_TIMER {}

impl MaximumPossibleSized for SMSG_PAUSE_MIRROR_TIMER {
    fn maximum_possible_size() -> usize {
        TimerType::size() // timer: TimerType
        + 1 // is_frozen: u8
    }
}

#[derive(Debug)]
pub enum SMSG_PAUSE_MIRROR_TIMERError {
    Io(std::io::Error),
    TimerType(TimerTypeError),
}

impl std::error::Error for SMSG_PAUSE_MIRROR_TIMERError {}
impl std::fmt::Display for SMSG_PAUSE_MIRROR_TIMERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TimerType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PAUSE_MIRROR_TIMERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TimerTypeError> for SMSG_PAUSE_MIRROR_TIMERError {
    fn from(e: TimerTypeError) -> Self {
        Self::TimerType(e)
    }
}

