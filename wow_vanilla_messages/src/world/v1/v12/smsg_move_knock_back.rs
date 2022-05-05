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
pub struct SMSG_MOVE_KNOCK_BACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub v_cos: f32,
    pub v_sin: f32,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
}

impl ServerMessageWrite for SMSG_MOVE_KNOCK_BACK {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_MOVE_KNOCK_BACK {
    const OPCODE: u16 = 0x00ef;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // v_cos: f32
        let v_cos = crate::util::read_f32_le(r)?;
        // v_sin: f32
        let v_sin = crate::util::read_f32_le(r)?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::read_f32_le(r)?;
        // vertical_speed: f32
        let vertical_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes())?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes())?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes())?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::tokio_read_packed(r).await?;

        // movement_counter: u32
        let movement_counter = crate::util::tokio_read_u32_le(r).await?;

        // v_cos: f32
        let v_cos = crate::util::tokio_read_f32_le(r).await?;
        // v_sin: f32
        let v_sin = crate::util::tokio_read_f32_le(r).await?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::tokio_read_f32_le(r).await?;
        // vertical_speed: f32
        let vertical_speed = crate::util::tokio_read_f32_le(r).await?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.tokio_write_packed(w).await?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes()).await?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes()).await?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes()).await?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes()).await?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::astd_read_packed(r).await?;

        // movement_counter: u32
        let movement_counter = crate::util::astd_read_u32_le(r).await?;

        // v_cos: f32
        let v_cos = crate::util::astd_read_f32_le(r).await?;
        // v_sin: f32
        let v_sin = crate::util::astd_read_f32_le(r).await?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::astd_read_f32_le(r).await?;
        // vertical_speed: f32
        let vertical_speed = crate::util::astd_read_f32_le(r).await?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.astd_write_packed(w).await?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes()).await?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes()).await?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes()).await?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes()).await?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes()).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_MOVE_KNOCK_BACK {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

impl MaximumPossibleSized for SMSG_MOVE_KNOCK_BACK {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

