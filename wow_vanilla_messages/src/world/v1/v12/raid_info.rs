use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct RaidInfo {
    pub map: Map,
    pub reset_time: u32,
    pub instance_id: u32,
}

impl RaidInfo {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(w)
    }
}

impl RaidInfo {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::read_u32_le(r)?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::tokio_read_u32_le(r).await?;

        // instance_id: u32
        let instance_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::astd_read_u32_le(r).await?;

        // instance_id: u32
        let instance_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

}

impl RaidInfo {
    pub(crate) fn size() -> usize {
        0
        + 4 // map: Map
        + 4 // reset_time: u32
        + 4 // instance_id: u32
    }
}

#[derive(Debug)]
pub enum RaidInfoError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for RaidInfoError {}
impl std::fmt::Display for RaidInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidInfoError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for RaidInfoError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

