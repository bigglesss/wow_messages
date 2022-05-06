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
pub struct SMSG_RESISTLOG {
    pub guid1: Guid,
    pub guid2: Guid,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl ServerMessageWrite for SMSG_RESISTLOG {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_RESISTLOG {
    const OPCODE: u16 = 0x01d6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: Guid
        let guid1 = Guid::read(r)?;

        // guid2: Guid
        let guid2 = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(r)?;
        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(r)?;
        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid1: Guid
        self.guid1.write(w)?;

        // guid2: Guid
        self.guid2.write(w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: Guid
        let guid1 = Guid::tokio_read(r).await?;

        // guid2: Guid
        let guid2 = Guid::tokio_read(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::tokio_read_u32_le(r).await?;

        // unknown2: f32
        let unknown2 = crate::util::tokio_read_f32_le(r).await?;
        // unknown3: f32
        let unknown3 = crate::util::tokio_read_f32_le(r).await?;
        // unknown4: u32
        let unknown4 = crate::util::tokio_read_u32_le(r).await?;

        // unknown5: u32
        let unknown5 = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid1: Guid
        self.guid1.tokio_write(w).await?;

        // guid2: Guid
        self.guid2.tokio_write(w).await?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes()).await?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes()).await?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes()).await?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: Guid
        let guid1 = Guid::astd_read(r).await?;

        // guid2: Guid
        let guid2 = Guid::astd_read(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::astd_read_u32_le(r).await?;

        // unknown2: f32
        let unknown2 = crate::util::astd_read_f32_le(r).await?;
        // unknown3: f32
        let unknown3 = crate::util::astd_read_f32_le(r).await?;
        // unknown4: u32
        let unknown4 = crate::util::astd_read_u32_le(r).await?;

        // unknown5: u32
        let unknown5 = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid1: Guid
        self.guid1.astd_write(w).await?;

        // guid2: Guid
        self.guid2.astd_write(w).await?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes()).await?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes()).await?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes()).await?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_RESISTLOG {}

impl MaximumPossibleSized for SMSG_RESISTLOG {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid1: Guid
        + 8 // guid2: Guid
        + 4 // unknown1: u32
        + 4 // unknown2: f32
        + 4 // unknown3: f32
        + 4 // unknown4: u32
        + 4 // unknown5: u32
    }
}

