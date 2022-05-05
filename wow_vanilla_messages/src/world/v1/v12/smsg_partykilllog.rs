use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_PARTYKILLLOG {
    pub player_with_killing_blow: Guid,
    pub victim: Guid,
}

impl ServerMessageWrite for SMSG_PARTYKILLLOG {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PARTYKILLLOG {
    const OPCODE: u16 = 0x01f5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::read(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        self.player_with_killing_blow.write(w)?;

        // victim: Guid
        self.victim.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::tokio_read(r).await?;

        // victim: Guid
        let victim = Guid::tokio_read(r).await?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        self.player_with_killing_blow.tokio_write(w).await?;

        // victim: Guid
        self.victim.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::astd_read(r).await?;

        // victim: Guid
        let victim = Guid::astd_read(r).await?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        self.player_with_killing_blow.astd_write(w).await?;

        // victim: Guid
        self.victim.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_PARTYKILLLOG {}

impl MaximumPossibleSized for SMSG_PARTYKILLLOG {
    fn maximum_possible_size() -> usize {
        8 // player_with_killing_blow: Guid
        + 8 // victim: Guid
    }
}

