use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::Vector3d;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector3d,
    pub orientation: f32,
    pub timestamp: u32,
}

impl TransportInfo {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        Ok(())
    }
}

impl TransportInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            position,
            orientation,
            timestamp,
        })
    }

}

impl TransportInfo {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 12 // position: Vector3d
        + 4 // orientation: f32
        + 4 // timestamp: u32
    }
}

