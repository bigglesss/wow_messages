use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InventoryResult {
    OK,
    CANT_EQUIP_LEVEL_I,
    CANT_EQUIP_SKILL,
    ITEM_DOESNT_GO_TO_SLOT,
    BAG_FULL,
    NONEMPTY_BAG_OVER_OTHER_BAG,
    CANT_TRADE_EQUIP_BAGS,
    ONLY_AMMO_CAN_GO_HERE,
    NO_REQUIRED_PROFICIENCY,
    NO_EQUIPMENT_SLOT_AVAILABLE,
    YOU_CAN_NEVER_USE_THAT_ITEM,
    YOU_CAN_NEVER_USE_THAT_ITEM2,
    NO_EQUIPMENT_SLOT_AVAILABLE2,
    CANT_EQUIP_WITH_TWOHANDED,
    CANT_DUAL_WIELD,
    ITEM_DOESNT_GO_INTO_BAG,
    ITEM_DOESNT_GO_INTO_BAG2,
    CANT_CARRY_MORE_OF_THIS,
    NO_EQUIPMENT_SLOT_AVAILABLE3,
    ITEM_CANT_STACK,
    ITEM_CANT_BE_EQUIPPED,
    ITEMS_CANT_BE_SWAPPED,
    SLOT_IS_EMPTY,
    ITEM_NOT_FOUND,
    CANT_DROP_SOULBOUND,
    OUT_OF_RANGE,
    TRIED_TO_SPLIT_MORE_THAN_COUNT,
    COULDNT_SPLIT_ITEMS,
    MISSING_REAGENT,
    NOT_ENOUGH_MONEY,
    NOT_A_BAG,
    CAN_ONLY_DO_WITH_EMPTY_BAGS,
    DONT_OWN_THAT_ITEM,
    CAN_EQUIP_ONLY1_QUIVER,
    MUST_PURCHASE_THAT_BAG_SLOT,
    TOO_FAR_AWAY_FROM_BANK,
    ITEM_LOCKED,
    YOU_ARE_STUNNED,
    YOU_ARE_DEAD,
    CANT_DO_RIGHT_NOW,
    INT_BAG_ERROR,
    CAN_EQUIP_ONLY1_BOLT,
    CAN_EQUIP_ONLY1_AMMOPOUCH,
    STACKABLE_CANT_BE_WRAPPED,
    EQUIPPED_CANT_BE_WRAPPED,
    WRAPPED_CANT_BE_WRAPPED,
    BOUND_CANT_BE_WRAPPED,
    UNIQUE_CANT_BE_WRAPPED,
    BAGS_CANT_BE_WRAPPED,
    ALREADY_LOOTED,
    INVENTORY_FULL,
    BANK_FULL,
    ITEM_IS_CURRENTLY_SOLD_OUT,
    BAG_FULL3,
    ITEM_NOT_FOUND2,
    ITEM_CANT_STACK2,
    BAG_FULL4,
    ITEM_SOLD_OUT,
    OBJECT_IS_BUSY,
    NONE,
    NOT_IN_COMBAT,
    NOT_WHILE_DISARMED,
    BAG_FULL6,
    CANT_EQUIP_RANK,
    CANT_EQUIP_REPUTATION,
    TOO_MANY_SPECIAL_BAGS,
    LOOT_CANT_LOOT_THAT_NOW,
}

impl ReadableAndWritable for InventoryResult {
    type Error = InventoryResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for InventoryResult {
    type Error = InventoryResultError;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes()).await?;
        Ok(())
    }

}

impl InventoryResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_le(w, self.as_u8() as u16).await?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(w, self.as_u8() as u16).await?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_u8() as u32).await?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_u8() as u32).await?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_u8() as u64).await?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InventoryResultError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_u8() as u64).await?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::OK => 0x0,
            Self::CANT_EQUIP_LEVEL_I => 0x1,
            Self::CANT_EQUIP_SKILL => 0x2,
            Self::ITEM_DOESNT_GO_TO_SLOT => 0x3,
            Self::BAG_FULL => 0x4,
            Self::NONEMPTY_BAG_OVER_OTHER_BAG => 0x5,
            Self::CANT_TRADE_EQUIP_BAGS => 0x6,
            Self::ONLY_AMMO_CAN_GO_HERE => 0x7,
            Self::NO_REQUIRED_PROFICIENCY => 0x8,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE => 0x9,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM => 0xa,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 => 0xb,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 => 0xc,
            Self::CANT_EQUIP_WITH_TWOHANDED => 0xd,
            Self::CANT_DUAL_WIELD => 0xe,
            Self::ITEM_DOESNT_GO_INTO_BAG => 0xf,
            Self::ITEM_DOESNT_GO_INTO_BAG2 => 0x10,
            Self::CANT_CARRY_MORE_OF_THIS => 0x11,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 => 0x12,
            Self::ITEM_CANT_STACK => 0x13,
            Self::ITEM_CANT_BE_EQUIPPED => 0x14,
            Self::ITEMS_CANT_BE_SWAPPED => 0x15,
            Self::SLOT_IS_EMPTY => 0x16,
            Self::ITEM_NOT_FOUND => 0x17,
            Self::CANT_DROP_SOULBOUND => 0x18,
            Self::OUT_OF_RANGE => 0x19,
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT => 0x1a,
            Self::COULDNT_SPLIT_ITEMS => 0x1b,
            Self::MISSING_REAGENT => 0x1c,
            Self::NOT_ENOUGH_MONEY => 0x1d,
            Self::NOT_A_BAG => 0x1e,
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS => 0x1f,
            Self::DONT_OWN_THAT_ITEM => 0x20,
            Self::CAN_EQUIP_ONLY1_QUIVER => 0x21,
            Self::MUST_PURCHASE_THAT_BAG_SLOT => 0x22,
            Self::TOO_FAR_AWAY_FROM_BANK => 0x23,
            Self::ITEM_LOCKED => 0x24,
            Self::YOU_ARE_STUNNED => 0x25,
            Self::YOU_ARE_DEAD => 0x26,
            Self::CANT_DO_RIGHT_NOW => 0x27,
            Self::INT_BAG_ERROR => 0x28,
            Self::CAN_EQUIP_ONLY1_BOLT => 0x29,
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH => 0x2a,
            Self::STACKABLE_CANT_BE_WRAPPED => 0x2b,
            Self::EQUIPPED_CANT_BE_WRAPPED => 0x2c,
            Self::WRAPPED_CANT_BE_WRAPPED => 0x2d,
            Self::BOUND_CANT_BE_WRAPPED => 0x2e,
            Self::UNIQUE_CANT_BE_WRAPPED => 0x2f,
            Self::BAGS_CANT_BE_WRAPPED => 0x30,
            Self::ALREADY_LOOTED => 0x31,
            Self::INVENTORY_FULL => 0x32,
            Self::BANK_FULL => 0x33,
            Self::ITEM_IS_CURRENTLY_SOLD_OUT => 0x34,
            Self::BAG_FULL3 => 0x35,
            Self::ITEM_NOT_FOUND2 => 0x36,
            Self::ITEM_CANT_STACK2 => 0x37,
            Self::BAG_FULL4 => 0x38,
            Self::ITEM_SOLD_OUT => 0x39,
            Self::OBJECT_IS_BUSY => 0x3a,
            Self::NONE => 0x3b,
            Self::NOT_IN_COMBAT => 0x3c,
            Self::NOT_WHILE_DISARMED => 0x3d,
            Self::BAG_FULL6 => 0x3e,
            Self::CANT_EQUIP_RANK => 0x3f,
            Self::CANT_EQUIP_REPUTATION => 0x40,
            Self::TOO_MANY_SPECIAL_BAGS => 0x41,
            Self::LOOT_CANT_LOOT_THAT_NOW => 0x42,
        }
    }

    pub const fn new() -> Self {
        Self::OK
    }

}

impl ConstantSized for InventoryResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for InventoryResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for InventoryResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for InventoryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::CANT_EQUIP_LEVEL_I => f.write_str("CANT_EQUIP_LEVEL_I"),
            Self::CANT_EQUIP_SKILL => f.write_str("CANT_EQUIP_SKILL"),
            Self::ITEM_DOESNT_GO_TO_SLOT => f.write_str("ITEM_DOESNT_GO_TO_SLOT"),
            Self::BAG_FULL => f.write_str("BAG_FULL"),
            Self::NONEMPTY_BAG_OVER_OTHER_BAG => f.write_str("NONEMPTY_BAG_OVER_OTHER_BAG"),
            Self::CANT_TRADE_EQUIP_BAGS => f.write_str("CANT_TRADE_EQUIP_BAGS"),
            Self::ONLY_AMMO_CAN_GO_HERE => f.write_str("ONLY_AMMO_CAN_GO_HERE"),
            Self::NO_REQUIRED_PROFICIENCY => f.write_str("NO_REQUIRED_PROFICIENCY"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE"),
            Self::YOU_CAN_NEVER_USE_THAT_ITEM => f.write_str("YOU_CAN_NEVER_USE_THAT_ITEM"),
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 => f.write_str("YOU_CAN_NEVER_USE_THAT_ITEM2"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE2"),
            Self::CANT_EQUIP_WITH_TWOHANDED => f.write_str("CANT_EQUIP_WITH_TWOHANDED"),
            Self::CANT_DUAL_WIELD => f.write_str("CANT_DUAL_WIELD"),
            Self::ITEM_DOESNT_GO_INTO_BAG => f.write_str("ITEM_DOESNT_GO_INTO_BAG"),
            Self::ITEM_DOESNT_GO_INTO_BAG2 => f.write_str("ITEM_DOESNT_GO_INTO_BAG2"),
            Self::CANT_CARRY_MORE_OF_THIS => f.write_str("CANT_CARRY_MORE_OF_THIS"),
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 => f.write_str("NO_EQUIPMENT_SLOT_AVAILABLE3"),
            Self::ITEM_CANT_STACK => f.write_str("ITEM_CANT_STACK"),
            Self::ITEM_CANT_BE_EQUIPPED => f.write_str("ITEM_CANT_BE_EQUIPPED"),
            Self::ITEMS_CANT_BE_SWAPPED => f.write_str("ITEMS_CANT_BE_SWAPPED"),
            Self::SLOT_IS_EMPTY => f.write_str("SLOT_IS_EMPTY"),
            Self::ITEM_NOT_FOUND => f.write_str("ITEM_NOT_FOUND"),
            Self::CANT_DROP_SOULBOUND => f.write_str("CANT_DROP_SOULBOUND"),
            Self::OUT_OF_RANGE => f.write_str("OUT_OF_RANGE"),
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT => f.write_str("TRIED_TO_SPLIT_MORE_THAN_COUNT"),
            Self::COULDNT_SPLIT_ITEMS => f.write_str("COULDNT_SPLIT_ITEMS"),
            Self::MISSING_REAGENT => f.write_str("MISSING_REAGENT"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NOT_A_BAG => f.write_str("NOT_A_BAG"),
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS => f.write_str("CAN_ONLY_DO_WITH_EMPTY_BAGS"),
            Self::DONT_OWN_THAT_ITEM => f.write_str("DONT_OWN_THAT_ITEM"),
            Self::CAN_EQUIP_ONLY1_QUIVER => f.write_str("CAN_EQUIP_ONLY1_QUIVER"),
            Self::MUST_PURCHASE_THAT_BAG_SLOT => f.write_str("MUST_PURCHASE_THAT_BAG_SLOT"),
            Self::TOO_FAR_AWAY_FROM_BANK => f.write_str("TOO_FAR_AWAY_FROM_BANK"),
            Self::ITEM_LOCKED => f.write_str("ITEM_LOCKED"),
            Self::YOU_ARE_STUNNED => f.write_str("YOU_ARE_STUNNED"),
            Self::YOU_ARE_DEAD => f.write_str("YOU_ARE_DEAD"),
            Self::CANT_DO_RIGHT_NOW => f.write_str("CANT_DO_RIGHT_NOW"),
            Self::INT_BAG_ERROR => f.write_str("INT_BAG_ERROR"),
            Self::CAN_EQUIP_ONLY1_BOLT => f.write_str("CAN_EQUIP_ONLY1_BOLT"),
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH => f.write_str("CAN_EQUIP_ONLY1_AMMOPOUCH"),
            Self::STACKABLE_CANT_BE_WRAPPED => f.write_str("STACKABLE_CANT_BE_WRAPPED"),
            Self::EQUIPPED_CANT_BE_WRAPPED => f.write_str("EQUIPPED_CANT_BE_WRAPPED"),
            Self::WRAPPED_CANT_BE_WRAPPED => f.write_str("WRAPPED_CANT_BE_WRAPPED"),
            Self::BOUND_CANT_BE_WRAPPED => f.write_str("BOUND_CANT_BE_WRAPPED"),
            Self::UNIQUE_CANT_BE_WRAPPED => f.write_str("UNIQUE_CANT_BE_WRAPPED"),
            Self::BAGS_CANT_BE_WRAPPED => f.write_str("BAGS_CANT_BE_WRAPPED"),
            Self::ALREADY_LOOTED => f.write_str("ALREADY_LOOTED"),
            Self::INVENTORY_FULL => f.write_str("INVENTORY_FULL"),
            Self::BANK_FULL => f.write_str("BANK_FULL"),
            Self::ITEM_IS_CURRENTLY_SOLD_OUT => f.write_str("ITEM_IS_CURRENTLY_SOLD_OUT"),
            Self::BAG_FULL3 => f.write_str("BAG_FULL3"),
            Self::ITEM_NOT_FOUND2 => f.write_str("ITEM_NOT_FOUND2"),
            Self::ITEM_CANT_STACK2 => f.write_str("ITEM_CANT_STACK2"),
            Self::BAG_FULL4 => f.write_str("BAG_FULL4"),
            Self::ITEM_SOLD_OUT => f.write_str("ITEM_SOLD_OUT"),
            Self::OBJECT_IS_BUSY => f.write_str("OBJECT_IS_BUSY"),
            Self::NONE => f.write_str("NONE"),
            Self::NOT_IN_COMBAT => f.write_str("NOT_IN_COMBAT"),
            Self::NOT_WHILE_DISARMED => f.write_str("NOT_WHILE_DISARMED"),
            Self::BAG_FULL6 => f.write_str("BAG_FULL6"),
            Self::CANT_EQUIP_RANK => f.write_str("CANT_EQUIP_RANK"),
            Self::CANT_EQUIP_REPUTATION => f.write_str("CANT_EQUIP_REPUTATION"),
            Self::TOO_MANY_SPECIAL_BAGS => f.write_str("TOO_MANY_SPECIAL_BAGS"),
            Self::LOOT_CANT_LOOT_THAT_NOW => f.write_str("LOOT_CANT_LOOT_THAT_NOW"),
        }
    }
}

impl TryFrom<u8> for InventoryResult {
    type Error = TryFromInventoryResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::CANT_EQUIP_LEVEL_I),
            2 => Ok(Self::CANT_EQUIP_SKILL),
            3 => Ok(Self::ITEM_DOESNT_GO_TO_SLOT),
            4 => Ok(Self::BAG_FULL),
            5 => Ok(Self::NONEMPTY_BAG_OVER_OTHER_BAG),
            6 => Ok(Self::CANT_TRADE_EQUIP_BAGS),
            7 => Ok(Self::ONLY_AMMO_CAN_GO_HERE),
            8 => Ok(Self::NO_REQUIRED_PROFICIENCY),
            9 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE),
            10 => Ok(Self::YOU_CAN_NEVER_USE_THAT_ITEM),
            11 => Ok(Self::YOU_CAN_NEVER_USE_THAT_ITEM2),
            12 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE2),
            13 => Ok(Self::CANT_EQUIP_WITH_TWOHANDED),
            14 => Ok(Self::CANT_DUAL_WIELD),
            15 => Ok(Self::ITEM_DOESNT_GO_INTO_BAG),
            16 => Ok(Self::ITEM_DOESNT_GO_INTO_BAG2),
            17 => Ok(Self::CANT_CARRY_MORE_OF_THIS),
            18 => Ok(Self::NO_EQUIPMENT_SLOT_AVAILABLE3),
            19 => Ok(Self::ITEM_CANT_STACK),
            20 => Ok(Self::ITEM_CANT_BE_EQUIPPED),
            21 => Ok(Self::ITEMS_CANT_BE_SWAPPED),
            22 => Ok(Self::SLOT_IS_EMPTY),
            23 => Ok(Self::ITEM_NOT_FOUND),
            24 => Ok(Self::CANT_DROP_SOULBOUND),
            25 => Ok(Self::OUT_OF_RANGE),
            26 => Ok(Self::TRIED_TO_SPLIT_MORE_THAN_COUNT),
            27 => Ok(Self::COULDNT_SPLIT_ITEMS),
            28 => Ok(Self::MISSING_REAGENT),
            29 => Ok(Self::NOT_ENOUGH_MONEY),
            30 => Ok(Self::NOT_A_BAG),
            31 => Ok(Self::CAN_ONLY_DO_WITH_EMPTY_BAGS),
            32 => Ok(Self::DONT_OWN_THAT_ITEM),
            33 => Ok(Self::CAN_EQUIP_ONLY1_QUIVER),
            34 => Ok(Self::MUST_PURCHASE_THAT_BAG_SLOT),
            35 => Ok(Self::TOO_FAR_AWAY_FROM_BANK),
            36 => Ok(Self::ITEM_LOCKED),
            37 => Ok(Self::YOU_ARE_STUNNED),
            38 => Ok(Self::YOU_ARE_DEAD),
            39 => Ok(Self::CANT_DO_RIGHT_NOW),
            40 => Ok(Self::INT_BAG_ERROR),
            41 => Ok(Self::CAN_EQUIP_ONLY1_BOLT),
            42 => Ok(Self::CAN_EQUIP_ONLY1_AMMOPOUCH),
            43 => Ok(Self::STACKABLE_CANT_BE_WRAPPED),
            44 => Ok(Self::EQUIPPED_CANT_BE_WRAPPED),
            45 => Ok(Self::WRAPPED_CANT_BE_WRAPPED),
            46 => Ok(Self::BOUND_CANT_BE_WRAPPED),
            47 => Ok(Self::UNIQUE_CANT_BE_WRAPPED),
            48 => Ok(Self::BAGS_CANT_BE_WRAPPED),
            49 => Ok(Self::ALREADY_LOOTED),
            50 => Ok(Self::INVENTORY_FULL),
            51 => Ok(Self::BANK_FULL),
            52 => Ok(Self::ITEM_IS_CURRENTLY_SOLD_OUT),
            53 => Ok(Self::BAG_FULL3),
            54 => Ok(Self::ITEM_NOT_FOUND2),
            55 => Ok(Self::ITEM_CANT_STACK2),
            56 => Ok(Self::BAG_FULL4),
            57 => Ok(Self::ITEM_SOLD_OUT),
            58 => Ok(Self::OBJECT_IS_BUSY),
            59 => Ok(Self::NONE),
            60 => Ok(Self::NOT_IN_COMBAT),
            61 => Ok(Self::NOT_WHILE_DISARMED),
            62 => Ok(Self::BAG_FULL6),
            63 => Ok(Self::CANT_EQUIP_RANK),
            64 => Ok(Self::CANT_EQUIP_REPUTATION),
            65 => Ok(Self::TOO_MANY_SPECIAL_BAGS),
            66 => Ok(Self::LOOT_CANT_LOOT_THAT_NOW),
            _ => Err(TryFromInventoryResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromInventoryResultError {
    value: u8,
}

impl TryFromInventoryResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum InventoryResultError {
    Read(std::io::Error),
    TryFrom(TryFromInventoryResultError),
}

impl std::error::Error for InventoryResultError {}
impl std::fmt::Display for TryFromInventoryResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'InventoryResult': '{}'", self.value))
    }
}

impl std::fmt::Display for InventoryResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for InventoryResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromInventoryResultError> for InventoryResultError {
    fn from(value: TryFromInventoryResultError) -> Self {
        Self::TryFrom(value)
    }
}

