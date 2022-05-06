use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{MeetingStoneStatus, MeetingStoneStatusError};
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
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl ServerMessageWrite for SMSG_MEETINGSTONE_SETQUEUE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u16 = 0x0295;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_MEETINGSTONE_SETQUEUEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area = Area::read(r)?;

        // status: MeetingStoneStatus
        let status = MeetingStoneStatus::read(r)?;

        Ok(Self {
            area,
            status,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        self.area.write(w)?;

        // status: MeetingStoneStatus
        self.status.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area = Area::tokio_read(r).await?;

        // status: MeetingStoneStatus
        let status = MeetingStoneStatus::tokio_read(r).await?;

        Ok(Self {
            area,
            status,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        self.area.tokio_write(w).await?;

        // status: MeetingStoneStatus
        self.status.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area = Area::astd_read(r).await?;

        // status: MeetingStoneStatus
        let status = MeetingStoneStatus::astd_read(r).await?;

        Ok(Self {
            area,
            status,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        self.area.astd_write(w).await?;

        // status: MeetingStoneStatus
        self.status.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_MEETINGSTONE_SETQUEUE {}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_SETQUEUE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // area: Area
        + 1 // status: MeetingStoneStatus
    }
}

#[derive(Debug)]
pub enum SMSG_MEETINGSTONE_SETQUEUEError {
    Io(std::io::Error),
    Area(AreaError),
    MeetingStoneStatus(MeetingStoneStatusError),
}

impl std::error::Error for SMSG_MEETINGSTONE_SETQUEUEError {}
impl std::fmt::Display for SMSG_MEETINGSTONE_SETQUEUEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::MeetingStoneStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MeetingStoneStatusError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: MeetingStoneStatusError) -> Self {
        Self::MeetingStoneStatus(e)
    }
}

