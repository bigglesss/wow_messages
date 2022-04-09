use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:75`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L75):
/// ```text
/// enum TradeStatus : u32 {
///     BUSY = 0;
///     BEGIN_TRADE = 1;
///     OPEN_WINDOW = 2;
///     TRADE_CANCELED = 3;
///     TRADE_ACCEPT = 4;
///     BUSY_2 = 5;
///     NO_TARGET = 6;
///     BACK_TO_TRADE = 7;
///     TRADE_COMPLETE = 8;
///     TRADE_REJECTED = 9;
///     TARGET_TO_FAR = 10;
///     WRONG_FACTION = 11;
///     CLOSE_WINDOW = 12;
///     UNKNOWN_13 = 13;
///     IGNORE_YOU = 14;
///     YOU_STUNNED = 15;
///     TARGET_STUNNED = 16;
///     YOU_DEAD = 17;
///     TARGET_DEAD = 18;
///     YOU_LOGOUT = 19;
///     TARGET_LOGOUT = 20;
///     TRIAL_ACCOUNT = 21;
///     ONLY_CONJURED = 22;
///     NOT_ON_TAPLIST = 23;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TradeStatus {
    BUSY,
    BEGIN_TRADE,
    OPEN_WINDOW,
    TRADE_CANCELED,
    TRADE_ACCEPT,
    BUSY_2,
    NO_TARGET,
    BACK_TO_TRADE,
    TRADE_COMPLETE,
    TRADE_REJECTED,
    TARGET_TO_FAR,
    WRONG_FACTION,
    CLOSE_WINDOW,
    UNKNOWN_13,
    IGNORE_YOU,
    YOU_STUNNED,
    TARGET_STUNNED,
    YOU_DEAD,
    TARGET_DEAD,
    YOU_LOGOUT,
    TARGET_LOGOUT,
    TRIAL_ACCOUNT,
    ONLY_CONJURED,
    NOT_ON_TAPLIST,
}

impl ReadableAndWritable for TradeStatus {
    type Error = TradeStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl TradeStatus {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TradeStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TradeStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TradeStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::BUSY => 0x0,
            Self::BEGIN_TRADE => 0x1,
            Self::OPEN_WINDOW => 0x2,
            Self::TRADE_CANCELED => 0x3,
            Self::TRADE_ACCEPT => 0x4,
            Self::BUSY_2 => 0x5,
            Self::NO_TARGET => 0x6,
            Self::BACK_TO_TRADE => 0x7,
            Self::TRADE_COMPLETE => 0x8,
            Self::TRADE_REJECTED => 0x9,
            Self::TARGET_TO_FAR => 0xa,
            Self::WRONG_FACTION => 0xb,
            Self::CLOSE_WINDOW => 0xc,
            Self::UNKNOWN_13 => 0xd,
            Self::IGNORE_YOU => 0xe,
            Self::YOU_STUNNED => 0xf,
            Self::TARGET_STUNNED => 0x10,
            Self::YOU_DEAD => 0x11,
            Self::TARGET_DEAD => 0x12,
            Self::YOU_LOGOUT => 0x13,
            Self::TARGET_LOGOUT => 0x14,
            Self::TRIAL_ACCOUNT => 0x15,
            Self::ONLY_CONJURED => 0x16,
            Self::NOT_ON_TAPLIST => 0x17,
        }
    }

    pub const fn new() -> Self {
        Self::BUSY
    }

}

impl ConstantSized for TradeStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TradeStatus {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for TradeStatus {
    fn default() -> Self {
        Self::BUSY
    }
}

impl std::fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BUSY => f.write_str("BUSY"),
            Self::BEGIN_TRADE => f.write_str("BEGIN_TRADE"),
            Self::OPEN_WINDOW => f.write_str("OPEN_WINDOW"),
            Self::TRADE_CANCELED => f.write_str("TRADE_CANCELED"),
            Self::TRADE_ACCEPT => f.write_str("TRADE_ACCEPT"),
            Self::BUSY_2 => f.write_str("BUSY_2"),
            Self::NO_TARGET => f.write_str("NO_TARGET"),
            Self::BACK_TO_TRADE => f.write_str("BACK_TO_TRADE"),
            Self::TRADE_COMPLETE => f.write_str("TRADE_COMPLETE"),
            Self::TRADE_REJECTED => f.write_str("TRADE_REJECTED"),
            Self::TARGET_TO_FAR => f.write_str("TARGET_TO_FAR"),
            Self::WRONG_FACTION => f.write_str("WRONG_FACTION"),
            Self::CLOSE_WINDOW => f.write_str("CLOSE_WINDOW"),
            Self::UNKNOWN_13 => f.write_str("UNKNOWN_13"),
            Self::IGNORE_YOU => f.write_str("IGNORE_YOU"),
            Self::YOU_STUNNED => f.write_str("YOU_STUNNED"),
            Self::TARGET_STUNNED => f.write_str("TARGET_STUNNED"),
            Self::YOU_DEAD => f.write_str("YOU_DEAD"),
            Self::TARGET_DEAD => f.write_str("TARGET_DEAD"),
            Self::YOU_LOGOUT => f.write_str("YOU_LOGOUT"),
            Self::TARGET_LOGOUT => f.write_str("TARGET_LOGOUT"),
            Self::TRIAL_ACCOUNT => f.write_str("TRIAL_ACCOUNT"),
            Self::ONLY_CONJURED => f.write_str("ONLY_CONJURED"),
            Self::NOT_ON_TAPLIST => f.write_str("NOT_ON_TAPLIST"),
        }
    }
}

impl TryFrom<u32> for TradeStatus {
    type Error = TryFromTradeStatusError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::BUSY),
            1 => Ok(Self::BEGIN_TRADE),
            2 => Ok(Self::OPEN_WINDOW),
            3 => Ok(Self::TRADE_CANCELED),
            4 => Ok(Self::TRADE_ACCEPT),
            5 => Ok(Self::BUSY_2),
            6 => Ok(Self::NO_TARGET),
            7 => Ok(Self::BACK_TO_TRADE),
            8 => Ok(Self::TRADE_COMPLETE),
            9 => Ok(Self::TRADE_REJECTED),
            10 => Ok(Self::TARGET_TO_FAR),
            11 => Ok(Self::WRONG_FACTION),
            12 => Ok(Self::CLOSE_WINDOW),
            13 => Ok(Self::UNKNOWN_13),
            14 => Ok(Self::IGNORE_YOU),
            15 => Ok(Self::YOU_STUNNED),
            16 => Ok(Self::TARGET_STUNNED),
            17 => Ok(Self::YOU_DEAD),
            18 => Ok(Self::TARGET_DEAD),
            19 => Ok(Self::YOU_LOGOUT),
            20 => Ok(Self::TARGET_LOGOUT),
            21 => Ok(Self::TRIAL_ACCOUNT),
            22 => Ok(Self::ONLY_CONJURED),
            23 => Ok(Self::NOT_ON_TAPLIST),
            _ => Err(TryFromTradeStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromTradeStatusError {
    value: u32,
}

impl TryFromTradeStatusError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum TradeStatusError {
    Read(std::io::Error),
    TryFrom(TryFromTradeStatusError),
}

impl std::error::Error for TradeStatusError {}
impl std::fmt::Display for TryFromTradeStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TradeStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for TradeStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TradeStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromTradeStatusError> for TradeStatusError {
    fn from(value: TryFromTradeStatusError) -> Self {
        Self::TryFrom(value)
    }
}
