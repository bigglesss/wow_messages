use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestItemRequirement {
    pub item: u32,
    pub item_count: u32,
    pub item_display_id: u32,
}

impl QuestItemRequirement {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl QuestItemRequirement {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::tokio_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::tokio_read_u32_le(r).await?;

        // item_display_id: u32
        let item_display_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::astd_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::astd_read_u32_le(r).await?;

        // item_display_id: u32
        let item_display_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

}

impl QuestItemRequirement {
    pub(crate) fn size() -> usize {
        0
        + 4 // item: u32
        + 4 // item_count: u32
        + 4 // item_display_id: u32
    }
}

