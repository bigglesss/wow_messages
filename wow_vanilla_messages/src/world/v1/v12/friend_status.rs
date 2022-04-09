use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L3):
/// ```text
/// enum FriendStatus : u8 {
///     OFFLINE = 0;
///     ONLINE = 1;
///     AFK = 2;
///     UNKNOWN3 = 3;
///     DND = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum FriendStatus {
    OFFLINE,
    ONLINE,
    AFK,
    UNKNOWN3,
    DND,
}

impl ReadableAndWritable for FriendStatus {
    type Error = FriendStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl FriendStatus {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::OFFLINE => 0x0,
            Self::ONLINE => 0x1,
            Self::AFK => 0x2,
            Self::UNKNOWN3 => 0x3,
            Self::DND => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::OFFLINE
    }

}

impl ConstantSized for FriendStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for FriendStatus {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for FriendStatus {
    fn default() -> Self {
        Self::OFFLINE
    }
}

impl std::fmt::Display for FriendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ONLINE => f.write_str("ONLINE"),
            Self::AFK => f.write_str("AFK"),
            Self::UNKNOWN3 => f.write_str("UNKNOWN3"),
            Self::DND => f.write_str("DND"),
        }
    }
}

impl TryFrom<u8> for FriendStatus {
    type Error = TryFromFriendStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OFFLINE),
            1 => Ok(Self::ONLINE),
            2 => Ok(Self::AFK),
            3 => Ok(Self::UNKNOWN3),
            4 => Ok(Self::DND),
            _ => Err(TryFromFriendStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromFriendStatusError {
    value: u8,
}

impl TryFromFriendStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum FriendStatusError {
    Read(std::io::Error),
    TryFrom(TryFromFriendStatusError),
}

impl std::error::Error for FriendStatusError {}
impl std::fmt::Display for TryFromFriendStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'FriendStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for FriendStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for FriendStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromFriendStatusError> for FriendStatusError {
    fn from(value: TryFromFriendStatusError) -> Self {
        Self::TryFrom(value)
    }
}
