use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:52`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L52):
/// ```text
/// enum GuildCommandResult : u8 {
///     PLAYER_NO_MORE_IN_GUILD = 0x00;
///     GUILD_INTERNAL = 0x01;
///     ALREADY_IN_GUILD = 0x02;
///     ALREADY_IN_GUILD_S = 0x03;
///     INVITED_TO_GUILD = 0x04;
///     ALREADY_INVITED_TO_GUILD_S = 0x05;
///     GUILD_NAME_INVALID = 0x06;
///     GUILD_NAME_EXISTS_S = 0x07;
///     GUILD_LEADER_LEAVE_OR_PERMISSIONS = 0x08;
///     GUILD_PLAYER_NOT_IN_GUILD = 0x09;
///     GUILD_PLAYER_NOT_IN_GUILD_S = 0x0A;
///     GUILD_PLAYER_NOT_FOUND_S = 0x0B;
///     GUILD_NOT_ALLIED = 0x0C;
///     GUILD_RANK_TOO_HIGH_S = 0x0D;
///     GUILD_RANK_TOO_LOW_S = 0x0E;
///     GUILD_RANKS_LOCKED = 0x11;
///     GUILD_RANK_IN_USE = 0x12;
///     GUILD_IGNORING_YOU_S = 0x13;
///     GUILD_UNK20 = 0x14;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildCommandResult {
    /// no message/error
    ///
    PlayerNoMoreInGuild,
    GuildInternal,
    AlreadyInGuild,
    AlreadyInGuildS,
    InvitedToGuild,
    AlreadyInvitedToGuildS,
    GuildNameInvalid,
    GuildNameExistsS,
    /// for Typecommand 0x03
    /// mangosone has both `GUILD_PERMISSIONS` and `GUILD_LEADER_LEAVE` as `0x08`.
    ///
    GuildLeaderLeaveOrPermissions,
    GuildPlayerNotInGuild,
    GuildPlayerNotInGuildS,
    GuildPlayerNotFoundS,
    GuildNotAllied,
    GuildRankTooHighS,
    GuildRankTooLowS,
    GuildRanksLocked,
    GuildRankInUse,
    GuildIgnoringYouS,
    /// for Typecommand 0x05 only
    ///
    GuildUnk20,
}

impl GuildCommandResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PlayerNoMoreInGuild => 0x0,
            Self::GuildInternal => 0x1,
            Self::AlreadyInGuild => 0x2,
            Self::AlreadyInGuildS => 0x3,
            Self::InvitedToGuild => 0x4,
            Self::AlreadyInvitedToGuildS => 0x5,
            Self::GuildNameInvalid => 0x6,
            Self::GuildNameExistsS => 0x7,
            Self::GuildLeaderLeaveOrPermissions => 0x8,
            Self::GuildPlayerNotInGuild => 0x9,
            Self::GuildPlayerNotInGuildS => 0xa,
            Self::GuildPlayerNotFoundS => 0xb,
            Self::GuildNotAllied => 0xc,
            Self::GuildRankTooHighS => 0xd,
            Self::GuildRankTooLowS => 0xe,
            Self::GuildRanksLocked => 0x11,
            Self::GuildRankInUse => 0x12,
            Self::GuildIgnoringYouS => 0x13,
            Self::GuildUnk20 => 0x14,
        }
    }

}

impl Default for GuildCommandResult {
    fn default() -> Self {
        Self::PlayerNoMoreInGuild
    }
}

impl std::fmt::Display for GuildCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerNoMoreInGuild => f.write_str("PlayerNoMoreInGuild"),
            Self::GuildInternal => f.write_str("GuildInternal"),
            Self::AlreadyInGuild => f.write_str("AlreadyInGuild"),
            Self::AlreadyInGuildS => f.write_str("AlreadyInGuildS"),
            Self::InvitedToGuild => f.write_str("InvitedToGuild"),
            Self::AlreadyInvitedToGuildS => f.write_str("AlreadyInvitedToGuildS"),
            Self::GuildNameInvalid => f.write_str("GuildNameInvalid"),
            Self::GuildNameExistsS => f.write_str("GuildNameExistsS"),
            Self::GuildLeaderLeaveOrPermissions => f.write_str("GuildLeaderLeaveOrPermissions"),
            Self::GuildPlayerNotInGuild => f.write_str("GuildPlayerNotInGuild"),
            Self::GuildPlayerNotInGuildS => f.write_str("GuildPlayerNotInGuildS"),
            Self::GuildPlayerNotFoundS => f.write_str("GuildPlayerNotFoundS"),
            Self::GuildNotAllied => f.write_str("GuildNotAllied"),
            Self::GuildRankTooHighS => f.write_str("GuildRankTooHighS"),
            Self::GuildRankTooLowS => f.write_str("GuildRankTooLowS"),
            Self::GuildRanksLocked => f.write_str("GuildRanksLocked"),
            Self::GuildRankInUse => f.write_str("GuildRankInUse"),
            Self::GuildIgnoringYouS => f.write_str("GuildIgnoringYouS"),
            Self::GuildUnk20 => f.write_str("GuildUnk20"),
        }
    }
}

impl TryFrom<u8> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PlayerNoMoreInGuild),
            1 => Ok(Self::GuildInternal),
            2 => Ok(Self::AlreadyInGuild),
            3 => Ok(Self::AlreadyInGuildS),
            4 => Ok(Self::InvitedToGuild),
            5 => Ok(Self::AlreadyInvitedToGuildS),
            6 => Ok(Self::GuildNameInvalid),
            7 => Ok(Self::GuildNameExistsS),
            8 => Ok(Self::GuildLeaderLeaveOrPermissions),
            9 => Ok(Self::GuildPlayerNotInGuild),
            10 => Ok(Self::GuildPlayerNotInGuildS),
            11 => Ok(Self::GuildPlayerNotFoundS),
            12 => Ok(Self::GuildNotAllied),
            13 => Ok(Self::GuildRankTooHighS),
            14 => Ok(Self::GuildRankTooLowS),
            17 => Ok(Self::GuildRanksLocked),
            18 => Ok(Self::GuildRankInUse),
            19 => Ok(Self::GuildIgnoringYouS),
            20 => Ok(Self::GuildUnk20),
            v => Err(crate::errors::EnumError::new("GuildCommandResult", v as u32),)
        }
    }
}

