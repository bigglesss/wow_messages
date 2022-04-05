use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:1028`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L1028):
/// ```text
/// struct BattlegroundPlayer {
///     Guid player;
///     PvpRank rank;
///     u32 killing_blows;
///     u32 honorable_kills;
///     u32 deaths;
///     u32 bonus_honor;
///     u32 amount_of_extra_fields;
///     u32[amount_of_extra_fields] fields;
/// }
/// ```
pub struct BattlegroundPlayer {
    pub player: Guid,
    pub rank: PvpRank,
    pub killing_blows: u32,
    pub honorable_kills: u32,
    pub deaths: u32,
    pub bonus_honor: u32,
    pub amount_of_extra_fields: u32,
    pub fields: Vec<u32>,
}

impl ReadableAndWritable for BattlegroundPlayer {
    type Error = BattlegroundPlayerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // player: Guid
        let player = Guid::read(r)?;

        // rank: PvpRank
        let rank = PvpRank::read_u32_le(r)?;

        // killing_blows: u32
        let killing_blows = crate::util::read_u32_le(r)?;

        // honorable_kills: u32
        let honorable_kills = crate::util::read_u32_le(r)?;

        // deaths: u32
        let deaths = crate::util::read_u32_le(r)?;

        // bonus_honor: u32
        let bonus_honor = crate::util::read_u32_le(r)?;

        // amount_of_extra_fields: u32
        let amount_of_extra_fields = crate::util::read_u32_le(r)?;

        // fields: u32[amount_of_extra_fields]
        let mut fields = Vec::with_capacity(amount_of_extra_fields as usize);
        for i in 0..amount_of_extra_fields {
            fields.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            player,
            rank,
            killing_blows,
            honorable_kills,
            deaths,
            bonus_honor,
            amount_of_extra_fields,
            fields,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: Guid
        self.player.write(w)?;

        // rank: PvpRank
        self.rank.write_u32_le(w)?;

        // killing_blows: u32
        w.write_all(&self.killing_blows.to_le_bytes())?;

        // honorable_kills: u32
        w.write_all(&self.honorable_kills.to_le_bytes())?;

        // deaths: u32
        w.write_all(&self.deaths.to_le_bytes())?;

        // bonus_honor: u32
        w.write_all(&self.bonus_honor.to_le_bytes())?;

        // amount_of_extra_fields: u32
        w.write_all(&(self.fields.len() as u32).to_le_bytes())?;

        // fields: u32[amount_of_extra_fields]
        for i in self.fields.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

}

impl VariableSized for BattlegroundPlayer {
    fn size(&self) -> usize {
        8 // player: Guid
        + 4 // rank: PvpRank upcasted to u32
        + 4 // killing_blows: u32
        + 4 // honorable_kills: u32
        + 4 // deaths: u32
        + 4 // bonus_honor: u32
        + 4 // amount_of_extra_fields: u32
        + self.fields.len() * core::mem::size_of::<u32>() // fields: u32[amount_of_extra_fields]
    }
}

impl MaximumPossibleSized for BattlegroundPlayer {
    fn maximum_possible_size() -> usize {
        8 // player: Guid
        + PvpRank::maximum_possible_size() // rank: PvpRank
        + 4 // killing_blows: u32
        + 4 // honorable_kills: u32
        + 4 // deaths: u32
        + 4 // bonus_honor: u32
        + 4 // amount_of_extra_fields: u32
        + 4294967295 * core::mem::size_of::<u32>() // fields: u32[amount_of_extra_fields]
    }
}

#[derive(Debug)]
pub enum BattlegroundPlayerError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for BattlegroundPlayerError {}
impl std::fmt::Display for BattlegroundPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for BattlegroundPlayerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for BattlegroundPlayerError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

