use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_messagechat.wowm#L3):
/// ```text
/// enum PlayerChatTag : u8 {
///     NONE = 0;
///     AFK = 1;
///     DND = 2;
///     GM = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PlayerChatTag {
    None,
    Afk,
    Dnd,
    Gm,
}

impl PlayerChatTag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Afk => 0x1,
            Self::Dnd => 0x2,
            Self::Gm => 0x3,
        }
    }

}

impl Default for PlayerChatTag {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for PlayerChatTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Afk => f.write_str("Afk"),
            Self::Dnd => f.write_str("Dnd"),
            Self::Gm => f.write_str("Gm"),
        }
    }
}

impl TryFrom<u8> for PlayerChatTag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Afk),
            2 => Ok(Self::Dnd),
            3 => Ok(Self::Gm),
            v => Err(crate::errors::EnumError::new("PlayerChatTag", v as u32),)
        }
    }
}

