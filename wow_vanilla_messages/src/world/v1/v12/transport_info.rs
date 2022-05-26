use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub orientation: f32,
    pub timestamp: u32,
}

impl TransportInfo {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        Ok(w)
    }
}

impl TransportInfo {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = Guid::tokio_read_packed(r).await?;

        // position_x: f32
        let position_x = crate::util::tokio_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::tokio_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::tokio_read_f32_le(r).await?;
        // orientation: f32
        let orientation = crate::util::tokio_read_f32_le(r).await?;
        // timestamp: u32
        let timestamp = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: PackedGuid
        let guid = Guid::astd_read_packed(r).await?;

        // position_x: f32
        let position_x = crate::util::astd_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::astd_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::astd_read_f32_le(r).await?;
        // orientation: f32
        let orientation = crate::util::astd_read_f32_le(r).await?;
        // timestamp: u32
        let timestamp = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

}

impl TransportInfo {
    pub fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
        + 4 // timestamp: u32
    }
}

