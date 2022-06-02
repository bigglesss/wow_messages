use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L3):
/// ```text
/// enum QuestPartyMessage : u8 {
///     SHARING_QUEST = 0;
///     CANT_TAKE_QUEST = 1;
///     ACCEPT_QUEST = 2;
///     DECLINE_QUEST = 3;
///     TOO_FAR = 4;
///     BUSY = 5;
///     LOG_FULL = 6;
///     HAVE_QUEST = 7;
///     FINISH_QUEST = 8;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestPartyMessage {
    SHARING_QUEST,
    CANT_TAKE_QUEST,
    ACCEPT_QUEST,
    DECLINE_QUEST,
    TOO_FAR,
    BUSY,
    LOG_FULL,
    HAVE_QUEST,
    FINISH_QUEST,
}

impl QuestPartyMessage {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SHARING_QUEST => 0x0,
            Self::CANT_TAKE_QUEST => 0x1,
            Self::ACCEPT_QUEST => 0x2,
            Self::DECLINE_QUEST => 0x3,
            Self::TOO_FAR => 0x4,
            Self::BUSY => 0x5,
            Self::LOG_FULL => 0x6,
            Self::HAVE_QUEST => 0x7,
            Self::FINISH_QUEST => 0x8,
        }
    }

}

impl Default for QuestPartyMessage {
    fn default() -> Self {
        Self::SHARING_QUEST
    }
}

impl std::fmt::Display for QuestPartyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHARING_QUEST => f.write_str("SHARING_QUEST"),
            Self::CANT_TAKE_QUEST => f.write_str("CANT_TAKE_QUEST"),
            Self::ACCEPT_QUEST => f.write_str("ACCEPT_QUEST"),
            Self::DECLINE_QUEST => f.write_str("DECLINE_QUEST"),
            Self::TOO_FAR => f.write_str("TOO_FAR"),
            Self::BUSY => f.write_str("BUSY"),
            Self::LOG_FULL => f.write_str("LOG_FULL"),
            Self::HAVE_QUEST => f.write_str("HAVE_QUEST"),
            Self::FINISH_QUEST => f.write_str("FINISH_QUEST"),
        }
    }
}

impl TryFrom<u8> for QuestPartyMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SHARING_QUEST),
            1 => Ok(Self::CANT_TAKE_QUEST),
            2 => Ok(Self::ACCEPT_QUEST),
            3 => Ok(Self::DECLINE_QUEST),
            4 => Ok(Self::TOO_FAR),
            5 => Ok(Self::BUSY),
            6 => Ok(Self::LOG_FULL),
            7 => Ok(Self::HAVE_QUEST),
            8 => Ok(Self::FINISH_QUEST),
            v => Err(crate::errors::EnumError::new("QuestPartyMessage", v as u32),)
        }
    }
}

