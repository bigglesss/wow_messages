use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_BATTLEMASTER_JOIN {
    pub guid: Guid,
    pub map: Map,
    pub instance_id: u32,
    pub join_as_group: u8,
}

impl ClientMessageWrite for CMSG_BATTLEMASTER_JOIN {}

impl MessageBody for CMSG_BATTLEMASTER_JOIN {
    const OPCODE: u16 = 0x02ee;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_BATTLEMASTER_JOINError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        // join_as_group: u8
        let join_as_group = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            map,
            instance_id,
            join_as_group,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // map: Map
        self.map.write(w)?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: u8
        w.write_all(&self.join_as_group.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // map: Map
            let map = Map::tokio_read(r).await?;

            // instance_id: u32
            let instance_id = crate::util::tokio_read_u32_le(r).await?;

            // join_as_group: u8
            let join_as_group = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                map,
                instance_id,
                join_as_group,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // map: Map
            self.map.tokio_write(w).await?;

            // instance_id: u32
            w.write_all(&self.instance_id.to_le_bytes()).await?;

            // join_as_group: u8
            w.write_all(&self.join_as_group.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // map: Map
            let map = Map::astd_read(r).await?;

            // instance_id: u32
            let instance_id = crate::util::astd_read_u32_le(r).await?;

            // join_as_group: u8
            let join_as_group = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                map,
                instance_id,
                join_as_group,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // map: Map
            self.map.astd_write(w).await?;

            // instance_id: u32
            w.write_all(&self.instance_id.to_le_bytes()).await?;

            // join_as_group: u8
            w.write_all(&self.join_as_group.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_BATTLEMASTER_JOIN {}

impl MaximumPossibleSized for CMSG_BATTLEMASTER_JOIN {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // map: Map
        + 4 // instance_id: u32
        + 1 // join_as_group: u8
    }
}

#[derive(Debug)]
pub enum CMSG_BATTLEMASTER_JOINError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for CMSG_BATTLEMASTER_JOINError {}
impl std::fmt::Display for CMSG_BATTLEMASTER_JOINError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BATTLEMASTER_JOINError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for CMSG_BATTLEMASTER_JOINError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

