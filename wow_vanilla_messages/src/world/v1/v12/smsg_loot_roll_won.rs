use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::RollVote;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOOT_ROLL_WON {
    pub looted_target_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub winning_player_guid: Guid,
    pub winning_roll: u8,
    pub vote: RollVote,
}

impl SMSG_LOOT_ROLL_WON {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: Guid
        w.write_all(&self.winning_player_guid.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for SMSG_LOOT_ROLL_WON {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: Guid
        w.write_all(&self.winning_player_guid.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x029f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        34
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // winning_player_guid: Guid
        let winning_player_guid = Guid::read(r)?;

        // winning_roll: u8
        let winning_roll = crate::util::read_u8_le(r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            looted_target_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            winning_player_guid,
            winning_roll,
            vote,
        })
    }

}

