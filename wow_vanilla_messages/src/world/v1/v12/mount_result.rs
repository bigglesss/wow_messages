use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/mount_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/mount_common.wowm#L3):
/// ```text
/// enum MountResult : u32 {
///     INVALIDMOUNTEE = 0;
///     TOOFARAWAY = 1;
///     ALREADYMOUNTED = 2;
///     NOTMOUNTABLE = 3;
///     NOTYOURPET = 4;
///     OTHER = 5;
///     LOOTING = 6;
///     RACECANTMOUNT = 7;
///     SHAPESHIFTED = 8;
///     FORCEDDISMOUNT = 9;
///     OK = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MountResult {
    INVALIDMOUNTEE,
    TOOFARAWAY,
    ALREADYMOUNTED,
    NOTMOUNTABLE,
    NOTYOURPET,
    OTHER,
    LOOTING,
    RACECANTMOUNT,
    SHAPESHIFTED,
    FORCEDDISMOUNT,
    OK,
}

impl ReadableAndWritable for MountResult {
    type Error = MountResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl MountResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MountResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MountResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MountResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::INVALIDMOUNTEE => 0x0,
            Self::TOOFARAWAY => 0x1,
            Self::ALREADYMOUNTED => 0x2,
            Self::NOTMOUNTABLE => 0x3,
            Self::NOTYOURPET => 0x4,
            Self::OTHER => 0x5,
            Self::LOOTING => 0x6,
            Self::RACECANTMOUNT => 0x7,
            Self::SHAPESHIFTED => 0x8,
            Self::FORCEDDISMOUNT => 0x9,
            Self::OK => 0xa,
        }
    }

    pub const fn new() -> Self {
        Self::INVALIDMOUNTEE
    }

}

impl ConstantSized for MountResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MountResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for MountResult {
    fn default() -> Self {
        Self::INVALIDMOUNTEE
    }
}

impl std::fmt::Display for MountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALIDMOUNTEE => f.write_str("INVALIDMOUNTEE"),
            Self::TOOFARAWAY => f.write_str("TOOFARAWAY"),
            Self::ALREADYMOUNTED => f.write_str("ALREADYMOUNTED"),
            Self::NOTMOUNTABLE => f.write_str("NOTMOUNTABLE"),
            Self::NOTYOURPET => f.write_str("NOTYOURPET"),
            Self::OTHER => f.write_str("OTHER"),
            Self::LOOTING => f.write_str("LOOTING"),
            Self::RACECANTMOUNT => f.write_str("RACECANTMOUNT"),
            Self::SHAPESHIFTED => f.write_str("SHAPESHIFTED"),
            Self::FORCEDDISMOUNT => f.write_str("FORCEDDISMOUNT"),
            Self::OK => f.write_str("OK"),
        }
    }
}

impl TryFrom<u32> for MountResult {
    type Error = TryFromMountResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::INVALIDMOUNTEE),
            1 => Ok(Self::TOOFARAWAY),
            2 => Ok(Self::ALREADYMOUNTED),
            3 => Ok(Self::NOTMOUNTABLE),
            4 => Ok(Self::NOTYOURPET),
            5 => Ok(Self::OTHER),
            6 => Ok(Self::LOOTING),
            7 => Ok(Self::RACECANTMOUNT),
            8 => Ok(Self::SHAPESHIFTED),
            9 => Ok(Self::FORCEDDISMOUNT),
            10 => Ok(Self::OK),
            _ => Err(TryFromMountResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMountResultError {
    value: u32,
}

impl TryFromMountResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MountResultError {
    Read(std::io::Error),
    TryFrom(TryFromMountResultError),
}

impl std::error::Error for MountResultError {}
impl std::fmt::Display for TryFromMountResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MountResult': '{}'", self.value))
    }
}

impl std::fmt::Display for MountResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MountResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMountResultError> for MountResultError {
    fn from(value: TryFromMountResultError) -> Self {
        Self::TryFrom(value)
    }
}
