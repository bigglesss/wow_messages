use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Map, MapError};
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
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub map: Map,
    pub unknown1: u8,
    pub unknown2: u32,
    pub unknown3: u8,
    pub battlegrounds: Vec<u32>,
}

impl ServerMessageWrite for SMSG_BATTLEFIELD_LIST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_BATTLEFIELD_LIST {
    const OPCODE: u16 = 0x023d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_BATTLEFIELD_LISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // battlemaster: Guid
        let battlemaster = Guid::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::read_u32_le(r)?;

        // battlegrounds: u32[number_of_battlegrounds]
        let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
        for i in 0..number_of_battlegrounds {
            battlegrounds.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            battlemaster,
            map,
            unknown1,
            unknown2,
            unknown3,
            battlegrounds,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // battlemaster: Guid
        self.battlemaster.write(w)?;

        // map: Map
        self.map.write(w)?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // battlemaster: Guid
        let battlemaster = Guid::tokio_read(r).await?;

        // map: Map
        let map = Map::tokio_read(r).await?;

        // unknown1: u8
        let unknown1 = crate::util::tokio_read_u8_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::tokio_read_u32_le(r).await?;

        // unknown3: u8
        let unknown3 = crate::util::tokio_read_u8_le(r).await?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::tokio_read_u32_le(r).await?;

        // battlegrounds: u32[number_of_battlegrounds]
        let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
        for i in 0..number_of_battlegrounds {
            battlegrounds.push(crate::util::tokio_read_u32_le(r).await?);
        }

        Ok(Self {
            battlemaster,
            map,
            unknown1,
            unknown2,
            unknown3,
            battlegrounds,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // battlemaster: Guid
        self.battlemaster.tokio_write(w).await?;

        // map: Map
        self.map.tokio_write(w).await?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes()).await?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes()).await?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes()).await?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // battlemaster: Guid
        let battlemaster = Guid::astd_read(r).await?;

        // map: Map
        let map = Map::astd_read(r).await?;

        // unknown1: u8
        let unknown1 = crate::util::astd_read_u8_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::astd_read_u32_le(r).await?;

        // unknown3: u8
        let unknown3 = crate::util::astd_read_u8_le(r).await?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::astd_read_u32_le(r).await?;

        // battlegrounds: u32[number_of_battlegrounds]
        let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
        for i in 0..number_of_battlegrounds {
            battlegrounds.push(crate::util::astd_read_u32_le(r).await?);
        }

        Ok(Self {
            battlemaster,
            map,
            unknown1,
            unknown2,
            unknown3,
            battlegrounds,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // battlemaster: Guid
        self.battlemaster.astd_write(w).await?;

        // map: Map
        self.map.astd_write(w).await?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes()).await?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes()).await?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes()).await?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_BATTLEFIELD_LIST {
    fn size(&self) -> usize {
        0
        + 8 // battlemaster: Guid
        + 4 // map: Map
        + 1 // unknown1: u8
        + 4 // unknown2: u32
        + 1 // unknown3: u8
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

impl MaximumPossibleSized for SMSG_BATTLEFIELD_LIST {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum SMSG_BATTLEFIELD_LISTError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_BATTLEFIELD_LISTError {}
impl std::fmt::Display for SMSG_BATTLEFIELD_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BATTLEFIELD_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_BATTLEFIELD_LISTError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

