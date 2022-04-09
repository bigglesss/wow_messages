use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L21):
/// ```text
/// enum ChatType : u8 {
///     SAY = 0x00;
///     PARTY = 0x01;
///     RAID = 0x02;
///     GUILD = 0x03;
///     OFFICER = 0x04;
///     YELL = 0x05;
///     WHISPER = 0x06;
///     WHISPER_INFORM = 0x07;
///     EMOTE = 0x08;
///     TEXT_EMOTE = 0x09;
///     SYSTEM = 0x0A;
///     MONSTER_SAY = 0x0B;
///     MONSTER_YELL = 0x0C;
///     MONSTER_EMOTE = 0x0D;
///     CHANNEL = 0x0E;
///     CHANNEL_JOIN = 0x0F;
///     CHANNEL_LEAVE = 0x10;
///     CHANNEL_LIST = 0x11;
///     CHANNEL_NOTICE = 0x12;
///     CHANNEL_NOTICE_USER = 0x13;
///     AFK = 0x14;
///     DND = 0x15;
///     IGNORED = 0x16;
///     SKILL = 0x17;
///     LOOT = 0x18;
///     MONSTER_WHISPER = 0x1A;
///     BG_SYSTEM_NEUTRAL = 0x52;
///     BG_SYSTEM_ALLIANCE = 0x53;
///     BG_SYSTEM_HORDE = 0x54;
///     RAID_LEADER = 0x57;
///     RAID_WARNING = 0x58;
///     RAID_BOSS_WHISPER = 0x59;
///     RAID_BOSS_EMOTE = 0x5A;
///     BATTLEGROUND = 0x5C;
///     BATTLEGROUND_LEADER = 0x5D;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ChatType {
    SAY,
    PARTY,
    RAID,
    GUILD,
    OFFICER,
    YELL,
    WHISPER,
    WHISPER_INFORM,
    EMOTE,
    TEXT_EMOTE,
    SYSTEM,
    MONSTER_SAY,
    MONSTER_YELL,
    MONSTER_EMOTE,
    CHANNEL,
    CHANNEL_JOIN,
    CHANNEL_LEAVE,
    CHANNEL_LIST,
    CHANNEL_NOTICE,
    CHANNEL_NOTICE_USER,
    AFK,
    DND,
    IGNORED,
    SKILL,
    LOOT,
    MONSTER_WHISPER,
    BG_SYSTEM_NEUTRAL,
    BG_SYSTEM_ALLIANCE,
    BG_SYSTEM_HORDE,
    RAID_LEADER,
    RAID_WARNING,
    RAID_BOSS_WHISPER,
    RAID_BOSS_EMOTE,
    BATTLEGROUND,
    BATTLEGROUND_LEADER,
}

impl ReadableAndWritable for ChatType {
    type Error = ChatTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl ChatType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ChatTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::SAY => 0x0,
            Self::PARTY => 0x1,
            Self::RAID => 0x2,
            Self::GUILD => 0x3,
            Self::OFFICER => 0x4,
            Self::YELL => 0x5,
            Self::WHISPER => 0x6,
            Self::WHISPER_INFORM => 0x7,
            Self::EMOTE => 0x8,
            Self::TEXT_EMOTE => 0x9,
            Self::SYSTEM => 0xa,
            Self::MONSTER_SAY => 0xb,
            Self::MONSTER_YELL => 0xc,
            Self::MONSTER_EMOTE => 0xd,
            Self::CHANNEL => 0xe,
            Self::CHANNEL_JOIN => 0xf,
            Self::CHANNEL_LEAVE => 0x10,
            Self::CHANNEL_LIST => 0x11,
            Self::CHANNEL_NOTICE => 0x12,
            Self::CHANNEL_NOTICE_USER => 0x13,
            Self::AFK => 0x14,
            Self::DND => 0x15,
            Self::IGNORED => 0x16,
            Self::SKILL => 0x17,
            Self::LOOT => 0x18,
            Self::MONSTER_WHISPER => 0x1a,
            Self::BG_SYSTEM_NEUTRAL => 0x52,
            Self::BG_SYSTEM_ALLIANCE => 0x53,
            Self::BG_SYSTEM_HORDE => 0x54,
            Self::RAID_LEADER => 0x57,
            Self::RAID_WARNING => 0x58,
            Self::RAID_BOSS_WHISPER => 0x59,
            Self::RAID_BOSS_EMOTE => 0x5a,
            Self::BATTLEGROUND => 0x5c,
            Self::BATTLEGROUND_LEADER => 0x5d,
        }
    }

    pub const fn new() -> Self {
        Self::SAY
    }

}

impl ConstantSized for ChatType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ChatType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for ChatType {
    fn default() -> Self {
        Self::SAY
    }
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SAY => f.write_str("SAY"),
            Self::PARTY => f.write_str("PARTY"),
            Self::RAID => f.write_str("RAID"),
            Self::GUILD => f.write_str("GUILD"),
            Self::OFFICER => f.write_str("OFFICER"),
            Self::YELL => f.write_str("YELL"),
            Self::WHISPER => f.write_str("WHISPER"),
            Self::WHISPER_INFORM => f.write_str("WHISPER_INFORM"),
            Self::EMOTE => f.write_str("EMOTE"),
            Self::TEXT_EMOTE => f.write_str("TEXT_EMOTE"),
            Self::SYSTEM => f.write_str("SYSTEM"),
            Self::MONSTER_SAY => f.write_str("MONSTER_SAY"),
            Self::MONSTER_YELL => f.write_str("MONSTER_YELL"),
            Self::MONSTER_EMOTE => f.write_str("MONSTER_EMOTE"),
            Self::CHANNEL => f.write_str("CHANNEL"),
            Self::CHANNEL_JOIN => f.write_str("CHANNEL_JOIN"),
            Self::CHANNEL_LEAVE => f.write_str("CHANNEL_LEAVE"),
            Self::CHANNEL_LIST => f.write_str("CHANNEL_LIST"),
            Self::CHANNEL_NOTICE => f.write_str("CHANNEL_NOTICE"),
            Self::CHANNEL_NOTICE_USER => f.write_str("CHANNEL_NOTICE_USER"),
            Self::AFK => f.write_str("AFK"),
            Self::DND => f.write_str("DND"),
            Self::IGNORED => f.write_str("IGNORED"),
            Self::SKILL => f.write_str("SKILL"),
            Self::LOOT => f.write_str("LOOT"),
            Self::MONSTER_WHISPER => f.write_str("MONSTER_WHISPER"),
            Self::BG_SYSTEM_NEUTRAL => f.write_str("BG_SYSTEM_NEUTRAL"),
            Self::BG_SYSTEM_ALLIANCE => f.write_str("BG_SYSTEM_ALLIANCE"),
            Self::BG_SYSTEM_HORDE => f.write_str("BG_SYSTEM_HORDE"),
            Self::RAID_LEADER => f.write_str("RAID_LEADER"),
            Self::RAID_WARNING => f.write_str("RAID_WARNING"),
            Self::RAID_BOSS_WHISPER => f.write_str("RAID_BOSS_WHISPER"),
            Self::RAID_BOSS_EMOTE => f.write_str("RAID_BOSS_EMOTE"),
            Self::BATTLEGROUND => f.write_str("BATTLEGROUND"),
            Self::BATTLEGROUND_LEADER => f.write_str("BATTLEGROUND_LEADER"),
        }
    }
}

impl TryFrom<u8> for ChatType {
    type Error = TryFromChatTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SAY),
            1 => Ok(Self::PARTY),
            2 => Ok(Self::RAID),
            3 => Ok(Self::GUILD),
            4 => Ok(Self::OFFICER),
            5 => Ok(Self::YELL),
            6 => Ok(Self::WHISPER),
            7 => Ok(Self::WHISPER_INFORM),
            8 => Ok(Self::EMOTE),
            9 => Ok(Self::TEXT_EMOTE),
            10 => Ok(Self::SYSTEM),
            11 => Ok(Self::MONSTER_SAY),
            12 => Ok(Self::MONSTER_YELL),
            13 => Ok(Self::MONSTER_EMOTE),
            14 => Ok(Self::CHANNEL),
            15 => Ok(Self::CHANNEL_JOIN),
            16 => Ok(Self::CHANNEL_LEAVE),
            17 => Ok(Self::CHANNEL_LIST),
            18 => Ok(Self::CHANNEL_NOTICE),
            19 => Ok(Self::CHANNEL_NOTICE_USER),
            20 => Ok(Self::AFK),
            21 => Ok(Self::DND),
            22 => Ok(Self::IGNORED),
            23 => Ok(Self::SKILL),
            24 => Ok(Self::LOOT),
            26 => Ok(Self::MONSTER_WHISPER),
            82 => Ok(Self::BG_SYSTEM_NEUTRAL),
            83 => Ok(Self::BG_SYSTEM_ALLIANCE),
            84 => Ok(Self::BG_SYSTEM_HORDE),
            87 => Ok(Self::RAID_LEADER),
            88 => Ok(Self::RAID_WARNING),
            89 => Ok(Self::RAID_BOSS_WHISPER),
            90 => Ok(Self::RAID_BOSS_EMOTE),
            92 => Ok(Self::BATTLEGROUND),
            93 => Ok(Self::BATTLEGROUND_LEADER),
            _ => Err(TryFromChatTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromChatTypeError {
    value: u8,
}

impl TryFromChatTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ChatTypeError {
    Read(std::io::Error),
    TryFrom(TryFromChatTypeError),
}

impl std::error::Error for ChatTypeError {}
impl std::fmt::Display for TryFromChatTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ChatType': '{}'", self.value))
    }
}

impl std::fmt::Display for ChatTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ChatTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromChatTypeError> for ChatTypeError {
    fn from(value: TryFromChatTypeError) -> Self {
        Self::TryFrom(value)
    }
}
