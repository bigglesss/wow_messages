use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{FriendStatus, FriendStatusError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L11):
/// ```text
/// struct Friend {
///     u64 guid;
///     FriendStatus status;
///     if (status != OFFLINE) {
///         Area area;
///         u32 level;
///         Class class;
///     }
/// }
/// ```
pub struct Friend {
    pub guid: u64,
    pub status: FriendFriendStatus,
}

impl ReadableAndWritable for Friend {
    type Error = FriendError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // status: FriendStatus
        let status = FriendStatus::read(r)?;

        let status_if = match status {
            FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::ONLINE {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::AFK => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::AFK {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::UNKNOWN3 => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::UNKNOWN3 {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::DND => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::DND {
                    area,
                    level,
                    class,
                }
            }
        };

        Ok(Self {
            guid,
            status: status_if,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // status: FriendStatus
        self.status.write(w)?;

        match &self.status {
            FriendFriendStatus::OFFLINE => {}
            FriendFriendStatus::ONLINE {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::AFK {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::DND {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
        }

        Ok(())
    }

}

impl VariableSized for Friend {
    fn size(&self) -> usize {
        8 // guid: u64
        + self.status.size() // status: FriendStatus and subfields
    }
}

impl MaximumPossibleSized for Friend {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + FriendStatus::maximum_possible_size() // status: FriendStatus
    }
}

#[derive(Debug)]
pub enum FriendError {
    Io(std::io::Error),
    Area(AreaError),
    Class(ClassError),
    FriendStatus(FriendStatusError),
}

impl std::error::Error for FriendError {}
impl std::fmt::Display for FriendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::FriendStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for FriendError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for FriendError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<ClassError> for FriendError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<FriendStatusError> for FriendError {
    fn from(e: FriendStatusError) -> Self {
        Self::FriendStatus(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FriendFriendStatus {
    OFFLINE,
    ONLINE {
        area: Area,
        level: u32,
        class: Class,
    },
    AFK {
        area: Area,
        level: u32,
        class: Class,
    },
    UNKNOWN3 {
        area: Area,
        level: u32,
        class: Class,
    },
    DND {
        area: Area,
        level: u32,
        class: Class,
    },
}

impl From<&FriendStatus> for FriendFriendStatus {
    fn from(e: &FriendStatus) -> Self {
        match &e {
            FriendStatus::OFFLINE => Self::OFFLINE,
            FriendStatus::ONLINE => Self::ONLINE {
                area: Default::default(),
                level: Default::default(),
                class: Default::default(),
            },
            FriendStatus::AFK => Self::AFK {
                area: Default::default(),
                level: Default::default(),
                class: Default::default(),
            },
            FriendStatus::UNKNOWN3 => Self::UNKNOWN3 {
                area: Default::default(),
                level: Default::default(),
                class: Default::default(),
            },
            FriendStatus::DND => Self::DND {
                area: Default::default(),
                level: Default::default(),
                class: Default::default(),
            },
        }
    }
}

impl From<&FriendFriendStatus> for FriendStatus {
    fn from(v: &FriendFriendStatus) -> Self {
        match &v {
            FriendFriendStatus::OFFLINE => Self::OFFLINE,
            FriendFriendStatus::ONLINE { .. } => Self::ONLINE,
            FriendFriendStatus::AFK { .. } => Self::AFK,
            FriendFriendStatus::UNKNOWN3 { .. } => Self::UNKNOWN3,
            FriendFriendStatus::DND { .. } => Self::DND,
        }
    }
}

impl Default for FriendFriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OFFLINE
    }
}

impl FriendFriendStatus {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for FriendFriendStatus {
    fn size(&self) -> usize {
        match self {
            Self::OFFLINE =>  {
                1
            }
            Self::ONLINE  {
                area,
                level,
                class,
            } => {
                1
                + Area::size() // area: Area
                + 4 // level: u32
                + 4 // class: Class upcasted to u32
            }
            Self::AFK  {
                area,
                level,
                class,
            } => {
                1
                + Area::size() // area: Area
                + 4 // level: u32
                + 4 // class: Class upcasted to u32
            }
            Self::UNKNOWN3  {
                area,
                level,
                class,
            } => {
                1
                + Area::size() // area: Area
                + 4 // level: u32
                + 4 // class: Class upcasted to u32
            }
            Self::DND  {
                area,
                level,
                class,
            } => {
                1
                + Area::size() // area: Area
                + 4 // level: u32
                + 4 // class: Class upcasted to u32
            }
        }
    }
}

impl MaximumPossibleSized for FriendFriendStatus {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}
