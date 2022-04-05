use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{InventoryType, InventoryTypeError};
use crate::world::v1::v12::{ItemClass, ItemClassError};
use crate::world::v1::v12::ItemDamageType;
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::world::v1::v12::ItemSpells;
use crate::world::v1::v12::ItemStat;
use crate::world::v1::v12::{Map, MapError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/9needs_optional/needs_optional.wowm:47`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/9needs_optional/needs_optional.wowm#L47):
/// ```text
/// smsg SMSG_ITEM_QUERY_SINGLE_RESPONSE = 0x58 {
///     u32 item;
///     OPTIONAL-STATEMENT-DOCC: unimplemented
/// }
/// ```
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub item: u32,
    pub found: Option<SMSG_ITEM_QUERY_SINGLE_RESPONSE_found>,
}

impl WorldServerMessageWrite for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    const OPCODE: u16 = 0x58;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    type Error = SMSG_ITEM_QUERY_SINGLE_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
            + 4 // item: u32
        };
        let found = if current_size < body_size as usize {
            // item_class: ItemClass
            let item_class = ItemClass::read_u32_le(r)?;

            // item_sub_class: u32
            let item_sub_class = crate::util::read_u32_le(r)?;

            // name1: CString
            let name1 = crate::util::read_c_string_to_vec(r)?;
            let name1 = String::from_utf8(name1)?;

            // name2: CString
            let name2 = crate::util::read_c_string_to_vec(r)?;
            let name2 = String::from_utf8(name2)?;

            // name3: CString
            let name3 = crate::util::read_c_string_to_vec(r)?;
            let name3 = String::from_utf8(name3)?;

            // name4: CString
            let name4 = crate::util::read_c_string_to_vec(r)?;
            let name4 = String::from_utf8(name4)?;

            // item_display_info: u32
            let item_display_info = crate::util::read_u32_le(r)?;

            // quality: ItemQuality
            let quality = ItemQuality::read_u32_le(r)?;

            // flags: u32
            let flags = crate::util::read_u32_le(r)?;

            // buy_price: f64
            let buy_price = crate::util::read_f64_le(r)?;
            // sell_price: f64
            let sell_price = crate::util::read_f64_le(r)?;
            // inventory_type: InventoryType
            let inventory_type = InventoryType::read(r)?;

            // allowed_class: u32
            let allowed_class = crate::util::read_u32_le(r)?;

            // allowed_race: u32
            let allowed_race = crate::util::read_u32_le(r)?;

            // item_level: u32
            let item_level = crate::util::read_u32_le(r)?;

            // required_level: u32
            let required_level = crate::util::read_u32_le(r)?;

            // required_skill: u32
            let required_skill = crate::util::read_u32_le(r)?;

            // required_skill_rank: u32
            let required_skill_rank = crate::util::read_u32_le(r)?;

            // required_spell: u32
            let required_spell = crate::util::read_u32_le(r)?;

            // required_honor_rank: u32
            let required_honor_rank = crate::util::read_u32_le(r)?;

            // required_city_rank: u32
            let required_city_rank = crate::util::read_u32_le(r)?;

            // required_reputation_faction: u32
            let required_reputation_faction = crate::util::read_u32_le(r)?;

            // required_reputation_rank: u32
            let required_reputation_rank = crate::util::read_u32_le(r)?;

            // max_count: u32
            let max_count = crate::util::read_u32_le(r)?;

            // stackable: u32
            let stackable = crate::util::read_u32_le(r)?;

            // container_slots: u32
            let container_slots = crate::util::read_u32_le(r)?;

            // stats: ItemStat[10]
            let mut stats = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                stats.push(ItemStat::read(r)?);
            }
            let stats = stats.try_into().unwrap();

            // damages: ItemDamageType[5]
            let mut damages = Vec::with_capacity(5 as usize);
            for i in 0..5 {
                damages.push(ItemDamageType::read(r)?);
            }
            let damages = damages.try_into().unwrap();

            // armor: u32
            let armor = crate::util::read_u32_le(r)?;

            // holy_resistance: u32
            let holy_resistance = crate::util::read_u32_le(r)?;

            // fire_resistance: u32
            let fire_resistance = crate::util::read_u32_le(r)?;

            // nature_resistance: u32
            let nature_resistance = crate::util::read_u32_le(r)?;

            // frost_resistance: u32
            let frost_resistance = crate::util::read_u32_le(r)?;

            // shadow_resistance: u32
            let shadow_resistance = crate::util::read_u32_le(r)?;

            // arcane_resistance: u32
            let arcane_resistance = crate::util::read_u32_le(r)?;

            // delay: u32
            let delay = crate::util::read_u32_le(r)?;

            // ammo_type: u32
            let ammo_type = crate::util::read_u32_le(r)?;

            // ranged_range_modification: f32
            let ranged_range_modification = crate::util::read_f32_le(r)?;
            // spells: ItemSpells[5]
            let mut spells = Vec::with_capacity(5 as usize);
            for i in 0..5 {
                spells.push(ItemSpells::read(r)?);
            }
            let spells = spells.try_into().unwrap();

            // bonding: u32
            let bonding = crate::util::read_u32_le(r)?;

            // description: CString
            let description = crate::util::read_c_string_to_vec(r)?;
            let description = String::from_utf8(description)?;

            // page_text: u32
            let page_text = crate::util::read_u32_le(r)?;

            // language_id: u32
            let language_id = crate::util::read_u32_le(r)?;

            // page_material: u32
            let page_material = crate::util::read_u32_le(r)?;

            // start_quest: u32
            let start_quest = crate::util::read_u32_le(r)?;

            // lock_id: u32
            let lock_id = crate::util::read_u32_le(r)?;

            // material: u32
            let material = crate::util::read_u32_le(r)?;

            // sheath: u32
            let sheath = crate::util::read_u32_le(r)?;

            // random_property: u32
            let random_property = crate::util::read_u32_le(r)?;

            // block: u32
            let block = crate::util::read_u32_le(r)?;

            // item_set: u32
            let item_set = crate::util::read_u32_le(r)?;

            // max_durability: u32
            let max_durability = crate::util::read_u32_le(r)?;

            // area: Area
            let area = Area::read(r)?;

            // map: Map
            let map = Map::read(r)?;

            // bag_family: u32
            let bag_family = crate::util::read_u32_le(r)?;

            Some(SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
                item_class,
                item_sub_class,
                name1,
                name2,
                name3,
                name4,
                item_display_info,
                quality,
                flags,
                buy_price,
                sell_price,
                inventory_type,
                allowed_class,
                allowed_race,
                item_level,
                required_level,
                required_skill,
                required_skill_rank,
                required_spell,
                required_honor_rank,
                required_city_rank,
                required_reputation_faction,
                required_reputation_rank,
                max_count,
                stackable,
                container_slots,
                stats,
                damages,
                armor,
                holy_resistance,
                fire_resistance,
                nature_resistance,
                frost_resistance,
                shadow_resistance,
                arcane_resistance,
                delay,
                ammo_type,
                ranged_range_modification,
                spells,
                bonding,
                description,
                page_text,
                language_id,
                page_material,
                start_quest,
                lock_id,
                material,
                sheath,
                random_property,
                block,
                item_set,
                max_durability,
                area,
                map,
                bag_family,
            })
        } else {
            None
        };

        Ok(Self {
            item,
            found,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // item_class: ItemClass
            v.item_class.write_u32_le(w)?;

            // item_sub_class: u32
            w.write_all(&v.item_sub_class.to_le_bytes())?;

            // name1: CString
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // item_display_info: u32
            w.write_all(&v.item_display_info.to_le_bytes())?;

            // quality: ItemQuality
            v.quality.write_u32_le(w)?;

            // flags: u32
            w.write_all(&v.flags.to_le_bytes())?;

            // buy_price: f64
            w.write_all(&v.buy_price.to_le_bytes())?;

            // sell_price: f64
            w.write_all(&v.sell_price.to_le_bytes())?;

            // inventory_type: InventoryType
            v.inventory_type.write(w)?;

            // allowed_class: u32
            w.write_all(&v.allowed_class.to_le_bytes())?;

            // allowed_race: u32
            w.write_all(&v.allowed_race.to_le_bytes())?;

            // item_level: u32
            w.write_all(&v.item_level.to_le_bytes())?;

            // required_level: u32
            w.write_all(&v.required_level.to_le_bytes())?;

            // required_skill: u32
            w.write_all(&v.required_skill.to_le_bytes())?;

            // required_skill_rank: u32
            w.write_all(&v.required_skill_rank.to_le_bytes())?;

            // required_spell: u32
            w.write_all(&v.required_spell.to_le_bytes())?;

            // required_honor_rank: u32
            w.write_all(&v.required_honor_rank.to_le_bytes())?;

            // required_city_rank: u32
            w.write_all(&v.required_city_rank.to_le_bytes())?;

            // required_reputation_faction: u32
            w.write_all(&v.required_reputation_faction.to_le_bytes())?;

            // required_reputation_rank: u32
            w.write_all(&v.required_reputation_rank.to_le_bytes())?;

            // max_count: u32
            w.write_all(&v.max_count.to_le_bytes())?;

            // stackable: u32
            w.write_all(&v.stackable.to_le_bytes())?;

            // container_slots: u32
            w.write_all(&v.container_slots.to_le_bytes())?;

            // stats: ItemStat[10]
            for i in v.stats.iter() {
                i.write(w)?;
            }

            // damages: ItemDamageType[5]
            for i in v.damages.iter() {
                i.write(w)?;
            }

            // armor: u32
            w.write_all(&v.armor.to_le_bytes())?;

            // holy_resistance: u32
            w.write_all(&v.holy_resistance.to_le_bytes())?;

            // fire_resistance: u32
            w.write_all(&v.fire_resistance.to_le_bytes())?;

            // nature_resistance: u32
            w.write_all(&v.nature_resistance.to_le_bytes())?;

            // frost_resistance: u32
            w.write_all(&v.frost_resistance.to_le_bytes())?;

            // shadow_resistance: u32
            w.write_all(&v.shadow_resistance.to_le_bytes())?;

            // arcane_resistance: u32
            w.write_all(&v.arcane_resistance.to_le_bytes())?;

            // delay: u32
            w.write_all(&v.delay.to_le_bytes())?;

            // ammo_type: u32
            w.write_all(&v.ammo_type.to_le_bytes())?;

            // ranged_range_modification: f32
            w.write_all(&v.ranged_range_modification.to_le_bytes())?;

            // spells: ItemSpells[5]
            for i in v.spells.iter() {
                i.write(w)?;
            }

            // bonding: u32
            w.write_all(&v.bonding.to_le_bytes())?;

            // description: CString
            w.write_all(v.description.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // page_text: u32
            w.write_all(&v.page_text.to_le_bytes())?;

            // language_id: u32
            w.write_all(&v.language_id.to_le_bytes())?;

            // page_material: u32
            w.write_all(&v.page_material.to_le_bytes())?;

            // start_quest: u32
            w.write_all(&v.start_quest.to_le_bytes())?;

            // lock_id: u32
            w.write_all(&v.lock_id.to_le_bytes())?;

            // material: u32
            w.write_all(&v.material.to_le_bytes())?;

            // sheath: u32
            w.write_all(&v.sheath.to_le_bytes())?;

            // random_property: u32
            w.write_all(&v.random_property.to_le_bytes())?;

            // block: u32
            w.write_all(&v.block.to_le_bytes())?;

            // item_set: u32
            w.write_all(&v.item_set.to_le_bytes())?;

            // max_durability: u32
            w.write_all(&v.max_durability.to_le_bytes())?;

            // area: Area
            v.area.write(w)?;

            // map: Map
            v.map.write(w)?;

            // bag_family: u32
            w.write_all(&v.bag_family.to_le_bytes())?;

        }

        Ok(())
    }
}

impl VariableSized for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn size(&self) -> usize {
        4 // item: u32
        + {
            if let Some(v) = &self.found {
                v.size()
            } else {
                0
            }
        } // optional found
    }
}

impl MaximumPossibleSized for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // item: u32
        + 65536 // optional found
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Area(AreaError),
    InventoryType(InventoryTypeError),
    ItemClass(ItemClassError),
    ItemQuality(ItemQualityError),
    Map(MapError),
}

impl std::error::Error for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::InventoryType(i) => i.fmt(f),
            Self::ItemClass(i) => i.fmt(f),
            Self::ItemQuality(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<AreaError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<InventoryTypeError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: InventoryTypeError) -> Self {
        Self::InventoryType(e)
    }
}

impl From<ItemClassError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: ItemClassError) -> Self {
        Self::ItemClass(e)
    }
}

impl From<ItemQualityError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: ItemQualityError) -> Self {
        Self::ItemQuality(e)
    }
}

impl From<MapError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
    pub item_class: ItemClass,
    pub item_sub_class: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub item_display_info: u32,
    pub quality: ItemQuality,
    pub flags: u32,
    pub buy_price: f64,
    pub sell_price: f64,
    pub inventory_type: InventoryType,
    pub allowed_class: u32,
    pub allowed_race: u32,
    pub item_level: u32,
    pub required_level: u32,
    pub required_skill: u32,
    pub required_skill_rank: u32,
    pub required_spell: u32,
    pub required_honor_rank: u32,
    pub required_city_rank: u32,
    pub required_reputation_faction: u32,
    pub required_reputation_rank: u32,
    pub max_count: u32,
    pub stackable: u32,
    pub container_slots: u32,
    pub stats: [ItemStat; 10],
    pub damages: [ItemDamageType; 5],
    pub armor: u32,
    pub holy_resistance: u32,
    pub fire_resistance: u32,
    pub nature_resistance: u32,
    pub frost_resistance: u32,
    pub shadow_resistance: u32,
    pub arcane_resistance: u32,
    pub delay: u32,
    pub ammo_type: u32,
    pub ranged_range_modification: f32,
    pub spells: [ItemSpells; 5],
    pub bonding: u32,
    pub description: String,
    pub page_text: u32,
    pub language_id: u32,
    pub page_material: u32,
    pub start_quest: u32,
    pub lock_id: u32,
    pub material: u32,
    pub sheath: u32,
    pub random_property: u32,
    pub block: u32,
    pub item_set: u32,
    pub max_durability: u32,
    pub area: Area,
    pub map: Map,
    pub bag_family: u32,
}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
    pub fn size(&self) -> usize {
        4 // item_class: ItemClass upcasted to u32
        + 4 // item_sub_class: u32
        + self.name1.len() + 1 // name1: CString and Null Terminator
        + self.name2.len() + 1 // name2: CString and Null Terminator
        + self.name3.len() + 1 // name3: CString and Null Terminator
        + self.name4.len() + 1 // name4: CString and Null Terminator
        + 4 // item_display_info: u32
        + 4 // quality: ItemQuality upcasted to u32
        + 4 // flags: u32
        + 8 // buy_price: f64
        + 8 // sell_price: f64
        + InventoryType::size() // inventory_type: InventoryType
        + 4 // allowed_class: u32
        + 4 // allowed_race: u32
        + 4 // item_level: u32
        + 4 // required_level: u32
        + 4 // required_skill: u32
        + 4 // required_skill_rank: u32
        + 4 // required_spell: u32
        + 4 // required_honor_rank: u32
        + 4 // required_city_rank: u32
        + 4 // required_reputation_faction: u32
        + 4 // required_reputation_rank: u32
        + 4 // max_count: u32
        + 4 // stackable: u32
        + 4 // container_slots: u32
        + 10 * ItemStat::size() // stats: ItemStat[10]
        + 5 * ItemDamageType::size() // damages: ItemDamageType[5]
        + 4 // armor: u32
        + 4 // holy_resistance: u32
        + 4 // fire_resistance: u32
        + 4 // nature_resistance: u32
        + 4 // frost_resistance: u32
        + 4 // shadow_resistance: u32
        + 4 // arcane_resistance: u32
        + 4 // delay: u32
        + 4 // ammo_type: u32
        + 4 // ranged_range_modification: f32
        + 5 * ItemSpells::size() // spells: ItemSpells[5]
        + 4 // bonding: u32
        + self.description.len() + 1 // description: CString and Null Terminator
        + 4 // page_text: u32
        + 4 // language_id: u32
        + 4 // page_material: u32
        + 4 // start_quest: u32
        + 4 // lock_id: u32
        + 4 // material: u32
        + 4 // sheath: u32
        + 4 // random_property: u32
        + 4 // block: u32
        + 4 // item_set: u32
        + 4 // max_durability: u32
        + Area::size() // area: Area
        + Map::size() // map: Map
        + 4 // bag_family: u32
    }
}

