use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::UpdateMask;
use crate::world::v1::v12::MovementBlock;
use crate::world::v1::v12::{ObjectType, ObjectTypeError};
use crate::world::v1::v12::{UpdateType, UpdateTypeError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub update_type: ObjectUpdateType,
}

impl ReadableAndWritable for Object {
    type Error = ObjectError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // update_type: UpdateType
        let update_type = UpdateType::read(r)?;

        let update_type_if = match update_type {
            UpdateType::VALUES => {
                // guid1: PackedGuid
                let guid1 = Guid::read_packed(r)?;

                // mask1: UpdateMask
                let mask1 = UpdateMask::read(r)?;

                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                }
            }
            UpdateType::MOVEMENT => {
                // guid2: PackedGuid
                let guid2 = Guid::read_packed(r)?;

                // movement1: MovementBlock
                let movement1 = MovementBlock::read(r)?;

                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                }
            }
            UpdateType::CREATE_OBJECT => {
                // guid3: PackedGuid
                let guid3 = Guid::read_packed(r)?;

                // object_type: ObjectType
                let object_type = ObjectType::read(r)?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(r)?;

                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::CREATE_OBJECT2 => {
                // guid3: PackedGuid
                let guid3 = Guid::read_packed(r)?;

                // object_type: ObjectType
                let object_type = ObjectType::read(r)?;

                // movement2: MovementBlock
                let movement2 = MovementBlock::read(r)?;

                // mask2: UpdateMask
                let mask2 = UpdateMask::read(r)?;

                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                }
            }
            UpdateType::OUT_OF_RANGE_OBJECTS => {
                // count: u32
                let count = crate::util::read_u32_le(r)?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::read_packed(r)?);
                }

                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                }
            }
            UpdateType::NEAR_OBJECTS => {
                // count: u32
                let count = crate::util::read_u32_le(r)?;

                // guids: PackedGuid[count]
                let mut guids = Vec::with_capacity(count as usize);
                for i in 0..count {
                    guids.push(Guid::read_packed(r)?);
                }

                ObjectUpdateType::NEAR_OBJECTS {
                    count,
                    guids,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // update_type: UpdateType
        self.update_type.write(w)?;

        match &self.update_type {
            ObjectUpdateType::VALUES {
                guid1,
                mask1,
            } => {
                // guid1: PackedGuid
                guid1.write_packed(w)?;

                // mask1: UpdateMask
                mask1.write(w)?;

            }
            ObjectUpdateType::MOVEMENT {
                guid2,
                movement1,
            } => {
                // guid2: PackedGuid
                guid2.write_packed(w)?;

                // movement1: MovementBlock
                movement1.write(w)?;

            }
            ObjectUpdateType::CREATE_OBJECT {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                guid3.write_packed(w)?;

                // object_type: ObjectType
                object_type.write(w)?;

                // movement2: MovementBlock
                movement2.write(w)?;

                // mask2: UpdateMask
                mask2.write(w)?;

            }
            ObjectUpdateType::CREATE_OBJECT2 {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                // guid3: PackedGuid
                guid3.write_packed(w)?;

                // object_type: ObjectType
                object_type.write(w)?;

                // movement2: MovementBlock
                movement2.write(w)?;

                // mask2: UpdateMask
                mask2.write(w)?;

            }
            ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                count,
                guids,
            } => {
                // count: u32
                w.write_all(&count.to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    i.write_packed(w)?;
                }

            }
            ObjectUpdateType::NEAR_OBJECTS {
                count,
                guids,
            } => {
                // count: u32
                w.write_all(&count.to_le_bytes())?;

                // guids: PackedGuid[count]
                for i in guids.iter() {
                    i.write_packed(w)?;
                }

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_type: UpdateType
            let update_type = UpdateType::tokio_read(r).await?;

            let update_type_if = match update_type {
                UpdateType::VALUES => {
                    // guid1: PackedGuid
                    let guid1 = Guid::tokio_read_packed(r).await?;

                    // mask1: UpdateMask
                    let mask1 = UpdateMask::tokio_read(r).await?;

                    ObjectUpdateType::VALUES {
                        guid1,
                        mask1,
                    }
                }
                UpdateType::MOVEMENT => {
                    // guid2: PackedGuid
                    let guid2 = Guid::tokio_read_packed(r).await?;

                    // movement1: MovementBlock
                    let movement1 = MovementBlock::tokio_read(r).await?;

                    ObjectUpdateType::MOVEMENT {
                        guid2,
                        movement1,
                    }
                }
                UpdateType::CREATE_OBJECT => {
                    // guid3: PackedGuid
                    let guid3 = Guid::tokio_read_packed(r).await?;

                    // object_type: ObjectType
                    let object_type = ObjectType::tokio_read(r).await?;

                    // movement2: MovementBlock
                    let movement2 = MovementBlock::tokio_read(r).await?;

                    // mask2: UpdateMask
                    let mask2 = UpdateMask::tokio_read(r).await?;

                    ObjectUpdateType::CREATE_OBJECT {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    }
                }
                UpdateType::CREATE_OBJECT2 => {
                    // guid3: PackedGuid
                    let guid3 = Guid::tokio_read_packed(r).await?;

                    // object_type: ObjectType
                    let object_type = ObjectType::tokio_read(r).await?;

                    // movement2: MovementBlock
                    let movement2 = MovementBlock::tokio_read(r).await?;

                    // mask2: UpdateMask
                    let mask2 = UpdateMask::tokio_read(r).await?;

                    ObjectUpdateType::CREATE_OBJECT2 {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    }
                }
                UpdateType::OUT_OF_RANGE_OBJECTS => {
                    // count: u32
                    let count = crate::util::tokio_read_u32_le(r).await?;

                    // guids: PackedGuid[count]
                    let mut guids = Vec::with_capacity(count as usize);
                    for i in 0..count {
                        guids.push(Guid::tokio_read_packed(r).await?);
                    }

                    ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                        count,
                        guids,
                    }
                }
                UpdateType::NEAR_OBJECTS => {
                    // count: u32
                    let count = crate::util::tokio_read_u32_le(r).await?;

                    // guids: PackedGuid[count]
                    let mut guids = Vec::with_capacity(count as usize);
                    for i in 0..count {
                        guids.push(Guid::tokio_read_packed(r).await?);
                    }

                    ObjectUpdateType::NEAR_OBJECTS {
                        count,
                        guids,
                    }
                }
            };

            Ok(Self {
                update_type: update_type_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            // update_type: UpdateType
            self.update_type.tokio_write(w).await?;

            match &self.update_type {
                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                } => {
                    // guid1: PackedGuid
                    guid1.tokio_write_packed(w).await?;

                    // mask1: UpdateMask
                    mask1.tokio_write(w).await?;

                }
                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                } => {
                    // guid2: PackedGuid
                    guid2.tokio_write_packed(w).await?;

                    // movement1: MovementBlock
                    movement1.tokio_write(w).await?;

                }
                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    // guid3: PackedGuid
                    guid3.tokio_write_packed(w).await?;

                    // object_type: ObjectType
                    object_type.tokio_write(w).await?;

                    // movement2: MovementBlock
                    movement2.tokio_write(w).await?;

                    // mask2: UpdateMask
                    mask2.tokio_write(w).await?;

                }
                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    // guid3: PackedGuid
                    guid3.tokio_write_packed(w).await?;

                    // object_type: ObjectType
                    object_type.tokio_write(w).await?;

                    // movement2: MovementBlock
                    movement2.tokio_write(w).await?;

                    // mask2: UpdateMask
                    mask2.tokio_write(w).await?;

                }
                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                } => {
                    // count: u32
                    w.write_all(&count.to_le_bytes()).await?;

                    // guids: PackedGuid[count]
                    for i in guids.iter() {
                        i.tokio_write_packed(w).await?;
                    }

                }
                ObjectUpdateType::NEAR_OBJECTS {
                    count,
                    guids,
                } => {
                    // count: u32
                    w.write_all(&count.to_le_bytes()).await?;

                    // guids: PackedGuid[count]
                    for i in guids.iter() {
                        i.tokio_write_packed(w).await?;
                    }

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_type: UpdateType
            let update_type = UpdateType::astd_read(r).await?;

            let update_type_if = match update_type {
                UpdateType::VALUES => {
                    // guid1: PackedGuid
                    let guid1 = Guid::astd_read_packed(r).await?;

                    // mask1: UpdateMask
                    let mask1 = UpdateMask::astd_read(r).await?;

                    ObjectUpdateType::VALUES {
                        guid1,
                        mask1,
                    }
                }
                UpdateType::MOVEMENT => {
                    // guid2: PackedGuid
                    let guid2 = Guid::astd_read_packed(r).await?;

                    // movement1: MovementBlock
                    let movement1 = MovementBlock::astd_read(r).await?;

                    ObjectUpdateType::MOVEMENT {
                        guid2,
                        movement1,
                    }
                }
                UpdateType::CREATE_OBJECT => {
                    // guid3: PackedGuid
                    let guid3 = Guid::astd_read_packed(r).await?;

                    // object_type: ObjectType
                    let object_type = ObjectType::astd_read(r).await?;

                    // movement2: MovementBlock
                    let movement2 = MovementBlock::astd_read(r).await?;

                    // mask2: UpdateMask
                    let mask2 = UpdateMask::astd_read(r).await?;

                    ObjectUpdateType::CREATE_OBJECT {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    }
                }
                UpdateType::CREATE_OBJECT2 => {
                    // guid3: PackedGuid
                    let guid3 = Guid::astd_read_packed(r).await?;

                    // object_type: ObjectType
                    let object_type = ObjectType::astd_read(r).await?;

                    // movement2: MovementBlock
                    let movement2 = MovementBlock::astd_read(r).await?;

                    // mask2: UpdateMask
                    let mask2 = UpdateMask::astd_read(r).await?;

                    ObjectUpdateType::CREATE_OBJECT2 {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    }
                }
                UpdateType::OUT_OF_RANGE_OBJECTS => {
                    // count: u32
                    let count = crate::util::astd_read_u32_le(r).await?;

                    // guids: PackedGuid[count]
                    let mut guids = Vec::with_capacity(count as usize);
                    for i in 0..count {
                        guids.push(Guid::astd_read_packed(r).await?);
                    }

                    ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                        count,
                        guids,
                    }
                }
                UpdateType::NEAR_OBJECTS => {
                    // count: u32
                    let count = crate::util::astd_read_u32_le(r).await?;

                    // guids: PackedGuid[count]
                    let mut guids = Vec::with_capacity(count as usize);
                    for i in 0..count {
                        guids.push(Guid::astd_read_packed(r).await?);
                    }

                    ObjectUpdateType::NEAR_OBJECTS {
                        count,
                        guids,
                    }
                }
            };

            Ok(Self {
                update_type: update_type_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            // update_type: UpdateType
            self.update_type.astd_write(w).await?;

            match &self.update_type {
                ObjectUpdateType::VALUES {
                    guid1,
                    mask1,
                } => {
                    // guid1: PackedGuid
                    guid1.astd_write_packed(w).await?;

                    // mask1: UpdateMask
                    mask1.astd_write(w).await?;

                }
                ObjectUpdateType::MOVEMENT {
                    guid2,
                    movement1,
                } => {
                    // guid2: PackedGuid
                    guid2.astd_write_packed(w).await?;

                    // movement1: MovementBlock
                    movement1.astd_write(w).await?;

                }
                ObjectUpdateType::CREATE_OBJECT {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    // guid3: PackedGuid
                    guid3.astd_write_packed(w).await?;

                    // object_type: ObjectType
                    object_type.astd_write(w).await?;

                    // movement2: MovementBlock
                    movement2.astd_write(w).await?;

                    // mask2: UpdateMask
                    mask2.astd_write(w).await?;

                }
                ObjectUpdateType::CREATE_OBJECT2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    // guid3: PackedGuid
                    guid3.astd_write_packed(w).await?;

                    // object_type: ObjectType
                    object_type.astd_write(w).await?;

                    // movement2: MovementBlock
                    movement2.astd_write(w).await?;

                    // mask2: UpdateMask
                    mask2.astd_write(w).await?;

                }
                ObjectUpdateType::OUT_OF_RANGE_OBJECTS {
                    count,
                    guids,
                } => {
                    // count: u32
                    w.write_all(&count.to_le_bytes()).await?;

                    // guids: PackedGuid[count]
                    for i in guids.iter() {
                        i.astd_write_packed(w).await?;
                    }

                }
                ObjectUpdateType::NEAR_OBJECTS {
                    count,
                    guids,
                } => {
                    // count: u32
                    w.write_all(&count.to_le_bytes()).await?;

                    // guids: PackedGuid[count]
                    for i in guids.iter() {
                        i.astd_write_packed(w).await?;
                    }

                }
            }

            Ok(())
        })
    }

}

impl VariableSized for Object {
    fn size(&self) -> usize {
        0
        + self.update_type.size() // update_type: ObjectUpdateType
    }
}

impl MaximumPossibleSized for Object {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum ObjectError {
    Io(std::io::Error),
    ObjectType(ObjectTypeError),
    UpdateType(UpdateTypeError),
}

impl std::error::Error for ObjectError {}
impl std::fmt::Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ObjectType(i) => i.fmt(f),
            Self::UpdateType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for ObjectError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ObjectTypeError> for ObjectError {
    fn from(e: ObjectTypeError) -> Self {
        Self::ObjectType(e)
    }
}

impl From<UpdateTypeError> for ObjectError {
    fn from(e: UpdateTypeError) -> Self {
        Self::UpdateType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectUpdateType {
    VALUES {
        guid1: Guid,
        mask1: UpdateMask,
    },
    MOVEMENT {
        guid2: Guid,
        movement1: MovementBlock,
    },
    CREATE_OBJECT {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    CREATE_OBJECT2 {
        guid3: Guid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    OUT_OF_RANGE_OBJECTS {
        count: u32,
        guids: Vec<Guid>,
    },
    NEAR_OBJECTS {
        count: u32,
        guids: Vec<Guid>,
    },
}

impl From<&UpdateType> for ObjectUpdateType {
    fn from(e: &UpdateType) -> Self {
        match &e {
            UpdateType::VALUES => Self::VALUES {
                guid1: Default::default(),
                mask1: Default::default(),
            },
            UpdateType::MOVEMENT => Self::MOVEMENT {
                guid2: Default::default(),
                movement1: Default::default(),
            },
            UpdateType::CREATE_OBJECT => Self::CREATE_OBJECT {
                guid3: Default::default(),
                mask2: Default::default(),
                movement2: Default::default(),
                object_type: Default::default(),
            },
            UpdateType::CREATE_OBJECT2 => Self::CREATE_OBJECT2 {
                guid3: Default::default(),
                mask2: Default::default(),
                movement2: Default::default(),
                object_type: Default::default(),
            },
            UpdateType::OUT_OF_RANGE_OBJECTS => Self::OUT_OF_RANGE_OBJECTS {
                count: Default::default(),
                guids: Default::default(),
            },
            UpdateType::NEAR_OBJECTS => Self::NEAR_OBJECTS {
                count: Default::default(),
                guids: Default::default(),
            },
        }
    }
}

impl From<&ObjectUpdateType> for UpdateType {
    fn from(v: &ObjectUpdateType) -> Self {
        match &v {
            ObjectUpdateType::VALUES { .. } => Self::VALUES,
            ObjectUpdateType::MOVEMENT { .. } => Self::MOVEMENT,
            ObjectUpdateType::CREATE_OBJECT { .. } => Self::CREATE_OBJECT,
            ObjectUpdateType::CREATE_OBJECT2 { .. } => Self::CREATE_OBJECT2,
            ObjectUpdateType::OUT_OF_RANGE_OBJECTS { .. } => Self::OUT_OF_RANGE_OBJECTS,
            ObjectUpdateType::NEAR_OBJECTS { .. } => Self::NEAR_OBJECTS,
        }
    }
}

impl Default for ObjectUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::VALUES {
            guid1: Default::default(),
            mask1: Default::default(),
        }
    }
}

impl ObjectUpdateType {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateType = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateType = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: UpdateType = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u8 {
        let a: UpdateType = self.into();
        a.as_int() as u8
    }

}

impl VariableSized for ObjectUpdateType {
    fn size(&self) -> usize {
        match self {
            Self::VALUES {
                guid1,
                mask1,
            } => {
                1
                + guid1.size() // guid1: Guid
                + mask1.size() // mask1: UpdateMask
            }
            Self::MOVEMENT {
                guid2,
                movement1,
            } => {
                1
                + guid2.size() // guid2: Guid
                + movement1.size() // movement1: MovementBlock
            }
            Self::CREATE_OBJECT {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                1
                + guid3.size() // guid3: Guid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::CREATE_OBJECT2 {
                guid3,
                mask2,
                movement2,
                object_type,
            } => {
                1
                + guid3.size() // guid3: Guid
                + mask2.size() // mask2: UpdateMask
                + movement2.size() // movement2: MovementBlock
                + 1 // object_type: ObjectType
            }
            Self::OUT_OF_RANGE_OBJECTS {
                count,
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + x.size()) // guids: PackedGuid[count]
            }
            Self::NEAR_OBJECTS {
                count,
                guids,
            } => {
                1
                + 4 // count: u32
                + guids.iter().fold(0, |acc, x| acc + x.size()) // guids: PackedGuid[count]
            }
        }
    }
}

impl MaximumPossibleSized for ObjectUpdateType {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

