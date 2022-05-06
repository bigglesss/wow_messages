use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::TradeSlot;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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

impl ServerMessageWrite for SMSG_TRADE_STATUS_EXTENDED {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_TRADE_STATUS_EXTENDED {
    const OPCODE: u16 = 0x0121;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
        for i in 0..7 {
            trade_slots[i] = TradeSlot::read(r)?;
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // self_player: u8
        let self_player = crate::util::tokio_read_u8_le(r).await?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::tokio_read_u32_le(r).await?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::tokio_read_u32_le(r).await?;

        // money_in_trade: u32
        let money_in_trade = crate::util::tokio_read_u32_le(r).await?;

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::tokio_read_u32_le(r).await?;

        // trade_slots: TradeSlot[7]
        let mut trade_slots = [TradeSlot::default(); 7];
        for i in 0..7 {
            trade_slots[i] = TradeSlot::tokio_read(r).await?;
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // self_player: u8
        w.write_all(&self.self_player.to_le_bytes()).await?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes()).await?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes()).await?;

        // money_in_trade: u32
        w.write_all(&self.money_in_trade.to_le_bytes()).await?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes()).await?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // self_player: u8
        let self_player = crate::util::astd_read_u8_le(r).await?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::astd_read_u32_le(r).await?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::astd_read_u32_le(r).await?;

        // money_in_trade: u32
        let money_in_trade = crate::util::astd_read_u32_le(r).await?;

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::astd_read_u32_le(r).await?;

        // trade_slots: TradeSlot[7]
        let mut trade_slots = [TradeSlot::default(); 7];
        for i in 0..7 {
            trade_slots[i] = TradeSlot::astd_read(r).await?;
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

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // self_player: u8
        w.write_all(&self.self_player.to_le_bytes()).await?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes()).await?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes()).await?;

        // money_in_trade: u32
        w.write_all(&self.money_in_trade.to_le_bytes()).await?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes()).await?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl ConstantSized for SMSG_TRADE_STATUS_EXTENDED {}

impl MaximumPossibleSized for SMSG_TRADE_STATUS_EXTENDED {
    fn maximum_possible_size() -> usize {
        0
        + 1 // self_player: u8
        + 4 // trade_slot_count1: u32
        + 4 // trade_slot_count2: u32
        + 4 // money_in_trade: u32
        + 4 // spell_on_lowest_slot: u32
        + 427 // trade_slots: TradeSlot[7]
    }
}

