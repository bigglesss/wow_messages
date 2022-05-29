use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::TradeSlot;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRADE_STATUS_EXTENDED {
    pub self_player: u8,
    pub trade_slot_count1: u32,
    pub trade_slot_count2: u32,
    pub money_in_trade: u32,
    pub spell_on_lowest_slot: u32,
    pub trade_slots: [TradeSlot; 7],
}

impl ServerMessage for SMSG_TRADE_STATUS_EXTENDED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // self_player: u8
        w.write_all(&self.self_player.to_le_bytes())?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes())?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes())?;

        // money_in_trade: u32
        w.write_all(&self.money_in_trade.to_le_bytes())?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes())?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0121;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        444
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // self_player: u8
        let self_player = crate::util::read_u8_le(r)?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::read_u32_le(r)?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::read_u32_le(r)?;

        // money_in_trade: u32
        let money_in_trade = crate::util::read_u32_le(r)?;

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::read_u32_le(r)?;

        // trade_slots: TradeSlot[7]
        let mut trade_slots = [TradeSlot::default(); 7];
        for i in trade_slots.iter_mut() {
            *i = TradeSlot::read(r)?;
        }

        Ok(Self {
            self_player,
            trade_slot_count1,
            trade_slot_count2,
            money_in_trade,
            spell_on_lowest_slot,
            trade_slots,
        })
    }

}

