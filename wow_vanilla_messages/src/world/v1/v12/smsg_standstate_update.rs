use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{UnitStandState, UnitStandStateError};
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
pub struct SMSG_STANDSTATE_UPDATE {
    pub state: UnitStandState,
}

impl ServerMessageWrite for SMSG_STANDSTATE_UPDATE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_STANDSTATE_UPDATE {
    const OPCODE: u16 = 0x029d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_STANDSTATE_UPDATEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: UnitStandState
        let state = UnitStandState::read(r)?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: UnitStandState
        self.state.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: UnitStandState
        let state = UnitStandState::tokio_read(r).await?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: UnitStandState
        self.state.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // state: UnitStandState
        let state = UnitStandState::astd_read(r).await?;

        Ok(Self {
            state,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: UnitStandState
        self.state.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_STANDSTATE_UPDATE {}

impl MaximumPossibleSized for SMSG_STANDSTATE_UPDATE {
    fn maximum_possible_size() -> usize {
        UnitStandState::size() // state: UnitStandState
    }
}

#[derive(Debug)]
pub enum SMSG_STANDSTATE_UPDATEError {
    Io(std::io::Error),
    UnitStandState(UnitStandStateError),
}

impl std::error::Error for SMSG_STANDSTATE_UPDATEError {}
impl std::fmt::Display for SMSG_STANDSTATE_UPDATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnitStandState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_STANDSTATE_UPDATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<UnitStandStateError> for SMSG_STANDSTATE_UPDATEError {
    fn from(e: UnitStandStateError) -> Self {
        Self::UnitStandState(e)
    }
}

