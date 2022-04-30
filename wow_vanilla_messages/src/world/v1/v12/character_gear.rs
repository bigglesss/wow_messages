use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InventoryType, InventoryTypeError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CharacterGear {
    type Error = CharacterGearError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::read_u32_le(r)?;

        // inventory_type: InventoryType
        let inventory_type = InventoryType::read(r)?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes())?;

        // inventory_type: InventoryType
        self.inventory_type.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::tokio_read_u32_le(r).await?;

        // inventory_type: InventoryType
        let inventory_type = InventoryType::tokio_read(r).await?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes()).await?;

        // inventory_type: InventoryType
        self.inventory_type.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::astd_read_u32_le(r).await?;

        // inventory_type: InventoryType
        let inventory_type = InventoryType::astd_read(r).await?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes()).await?;

        // inventory_type: InventoryType
        self.inventory_type.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for CharacterGear {}

impl MaximumPossibleSized for CharacterGear {
    fn maximum_possible_size() -> usize {
        4 // equipment_display_id: u32
        + InventoryType::size() // inventory_type: InventoryType
    }
}

#[derive(Debug)]
pub enum CharacterGearError {
    Io(std::io::Error),
    InventoryType(InventoryTypeError),
}

impl std::error::Error for CharacterGearError {}
impl std::fmt::Display for CharacterGearError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InventoryType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CharacterGearError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InventoryTypeError> for CharacterGearError {
    fn from(e: InventoryTypeError) -> Self {
        Self::InventoryType(e)
    }
}

