use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:122`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L122):
/// ```text
/// enum ItemClass : u8 {
///     CONSUMABLE = 0;
///     CONTAINER = 1;
///     WEAPON = 2;
///     RESERVED_1 = 3;
///     ARMOR = 4;
///     REAGENT = 5;
///     PROJECTILE = 6;
///     TRADE_GOODS = 7;
///     RESERVED_2 = 8;
///     RECIPE = 9;
///     RESERVED_3 = 10;
///     QUIVER = 11;
///     QUEST = 12;
///     KEY = 13;
///     RESERVED_4 = 14;
///     MISC = 15;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemClass {
    Consumable,
    Container,
    Weapon,
    Reserved1,
    Armor,
    Reagent,
    Projectile,
    TradeGoods,
    Reserved2,
    Recipe,
    Reserved3,
    Quiver,
    Quest,
    Key,
    Reserved4,
    Misc,
}

impl ItemClass {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Consumable => 0x0,
            Self::Container => 0x1,
            Self::Weapon => 0x2,
            Self::Reserved1 => 0x3,
            Self::Armor => 0x4,
            Self::Reagent => 0x5,
            Self::Projectile => 0x6,
            Self::TradeGoods => 0x7,
            Self::Reserved2 => 0x8,
            Self::Recipe => 0x9,
            Self::Reserved3 => 0xa,
            Self::Quiver => 0xb,
            Self::Quest => 0xc,
            Self::Key => 0xd,
            Self::Reserved4 => 0xe,
            Self::Misc => 0xf,
        }
    }

}

impl Default for ItemClass {
    fn default() -> Self {
        Self::Consumable
    }
}

impl std::fmt::Display for ItemClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Consumable => f.write_str("Consumable"),
            Self::Container => f.write_str("Container"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Reserved1 => f.write_str("Reserved1"),
            Self::Armor => f.write_str("Armor"),
            Self::Reagent => f.write_str("Reagent"),
            Self::Projectile => f.write_str("Projectile"),
            Self::TradeGoods => f.write_str("TradeGoods"),
            Self::Reserved2 => f.write_str("Reserved2"),
            Self::Recipe => f.write_str("Recipe"),
            Self::Reserved3 => f.write_str("Reserved3"),
            Self::Quiver => f.write_str("Quiver"),
            Self::Quest => f.write_str("Quest"),
            Self::Key => f.write_str("Key"),
            Self::Reserved4 => f.write_str("Reserved4"),
            Self::Misc => f.write_str("Misc"),
        }
    }
}

impl TryFrom<u8> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Consumable),
            1 => Ok(Self::Container),
            2 => Ok(Self::Weapon),
            3 => Ok(Self::Reserved1),
            4 => Ok(Self::Armor),
            5 => Ok(Self::Reagent),
            6 => Ok(Self::Projectile),
            7 => Ok(Self::TradeGoods),
            8 => Ok(Self::Reserved2),
            9 => Ok(Self::Recipe),
            10 => Ok(Self::Reserved3),
            11 => Ok(Self::Quiver),
            12 => Ok(Self::Quest),
            13 => Ok(Self::Key),
            14 => Ok(Self::Reserved4),
            15 => Ok(Self::Misc),
            v => Err(crate::errors::EnumError::new("ItemClass", v as u32),)
        }
    }
}

