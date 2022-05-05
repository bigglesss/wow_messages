use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct GossipItem {
    pub id: u32,
    pub item_icon: u8,
    pub coded: u8,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for GossipItem {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: u8
        let coded = crate::util::read_u8_le(r)?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::tokio_read_u32_le(r).await?;

        // item_icon: u8
        let item_icon = crate::util::tokio_read_u8_le(r).await?;

        // coded: u8
        let coded = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes()).await?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes()).await?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::astd_read_u32_le(r).await?;

        // item_icon: u8
        let item_icon = crate::util::astd_read_u8_le(r).await?;

        // coded: u8
        let coded = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes()).await?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes()).await?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for GossipItem {}

impl MaximumPossibleSized for GossipItem {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 1 // item_icon: u8
        + 1 // coded: u8
    }
}

