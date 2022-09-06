use crate::Guid;
use crate::wrath::{Race};
use crate::wrath::{Class};
use crate::wrath::{Gender};
use crate::wrath::{Power};
use crate::wrath::{UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdateMask, UpdatePlayer, UpdateUnit};

impl UpdateItem {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_3(mut self, v: u32) -> Self {
        self.header_set(24);
        self.values.insert(24, v);
        self
    }

    pub fn set_item_ENCHANTMENT_2_1(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_3(mut self, v: u32) -> Self {
        self.header_set(27);
        self.values.insert(27, v);
        self
    }

    pub fn set_item_ENCHANTMENT_3_1(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_3(mut self, v: u32) -> Self {
        self.header_set(30);
        self.values.insert(30, v);
        self
    }

    pub fn set_item_ENCHANTMENT_4_1(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_3(mut self, v: u32) -> Self {
        self.header_set(33);
        self.values.insert(33, v);
        self
    }

    pub fn set_item_ENCHANTMENT_5_1(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_3(mut self, v: u32) -> Self {
        self.header_set(36);
        self.values.insert(36, v);
        self
    }

    pub fn set_item_ENCHANTMENT_6_1(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_3(mut self, v: u32) -> Self {
        self.header_set(39);
        self.values.insert(39, v);
        self
    }

    pub fn set_item_ENCHANTMENT_7_1(mut self, v: i32) -> Self {
        self.header_set(40);
        self.values.insert(40, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_3(mut self, v: u32) -> Self {
        self.header_set(42);
        self.values.insert(42, v);
        self
    }

    pub fn set_item_ENCHANTMENT_8_1(mut self, v: i32) -> Self {
        self.header_set(43);
        self.values.insert(43, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_3(mut self, v: u32) -> Self {
        self.header_set(45);
        self.values.insert(45, v);
        self
    }

    pub fn set_item_ENCHANTMENT_9_1(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_3(mut self, v: u32) -> Self {
        self.header_set(48);
        self.values.insert(48, v);
        self
    }

    pub fn set_item_ENCHANTMENT_10_1(mut self, v: i32) -> Self {
        self.header_set(49);
        self.values.insert(49, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_3(mut self, v: u32) -> Self {
        self.header_set(51);
        self.values.insert(51, v);
        self
    }

    pub fn set_item_ENCHANTMENT_11_1(mut self, v: i32) -> Self {
        self.header_set(52);
        self.values.insert(52, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_3(mut self, v: u32) -> Self {
        self.header_set(54);
        self.values.insert(54, v);
        self
    }

    pub fn set_item_ENCHANTMENT_12_1(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_3(mut self, v: u32) -> Self {
        self.header_set(57);
        self.values.insert(57, v);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(58);
        self.values.insert(58, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(60);
        self.values.insert(60, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(61);
        self.values.insert(61, v as u32);
        self
    }

    pub fn set_item_CREATE_PLAYED_TIME(mut self, v: i32) -> Self {
        self.header_set(62);
        self.values.insert(62, v as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateContainer {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_3(mut self, v: u32) -> Self {
        self.header_set(24);
        self.values.insert(24, v);
        self
    }

    pub fn set_item_ENCHANTMENT_2_1(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_3(mut self, v: u32) -> Self {
        self.header_set(27);
        self.values.insert(27, v);
        self
    }

    pub fn set_item_ENCHANTMENT_3_1(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_3(mut self, v: u32) -> Self {
        self.header_set(30);
        self.values.insert(30, v);
        self
    }

    pub fn set_item_ENCHANTMENT_4_1(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_3(mut self, v: u32) -> Self {
        self.header_set(33);
        self.values.insert(33, v);
        self
    }

    pub fn set_item_ENCHANTMENT_5_1(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_3(mut self, v: u32) -> Self {
        self.header_set(36);
        self.values.insert(36, v);
        self
    }

    pub fn set_item_ENCHANTMENT_6_1(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_3(mut self, v: u32) -> Self {
        self.header_set(39);
        self.values.insert(39, v);
        self
    }

    pub fn set_item_ENCHANTMENT_7_1(mut self, v: i32) -> Self {
        self.header_set(40);
        self.values.insert(40, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_3(mut self, v: u32) -> Self {
        self.header_set(42);
        self.values.insert(42, v);
        self
    }

    pub fn set_item_ENCHANTMENT_8_1(mut self, v: i32) -> Self {
        self.header_set(43);
        self.values.insert(43, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_3(mut self, v: u32) -> Self {
        self.header_set(45);
        self.values.insert(45, v);
        self
    }

    pub fn set_item_ENCHANTMENT_9_1(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_3(mut self, v: u32) -> Self {
        self.header_set(48);
        self.values.insert(48, v);
        self
    }

    pub fn set_item_ENCHANTMENT_10_1(mut self, v: i32) -> Self {
        self.header_set(49);
        self.values.insert(49, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_3(mut self, v: u32) -> Self {
        self.header_set(51);
        self.values.insert(51, v);
        self
    }

    pub fn set_item_ENCHANTMENT_11_1(mut self, v: i32) -> Self {
        self.header_set(52);
        self.values.insert(52, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_3(mut self, v: u32) -> Self {
        self.header_set(54);
        self.values.insert(54, v);
        self
    }

    pub fn set_item_ENCHANTMENT_12_1(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_3(mut self, v: u32) -> Self {
        self.header_set(57);
        self.values.insert(57, v);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(58);
        self.values.insert(58, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(60);
        self.values.insert(60, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(61);
        self.values.insert(61, v as u32);
        self
    }

    pub fn set_item_CREATE_PLAYED_TIME(mut self, v: i32) -> Self {
        self.header_set(62);
        self.values.insert(62, v as u32);
        self
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(64);
        self.values.insert(64, v as u32);
        self
    }

    pub fn set_container_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(66);
        self.header_set(67);
        self.values.insert(66, v.guid() as u32);
        self.values.insert(67, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnit {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CRITTER(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23);
        self.values.insert(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn set_unit_POWER6(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn set_unit_POWER7(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(36);
        self.values.insert(36, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER6(mut self, v: i32) -> Self {
        self.header_set(38);
        self.values.insert(38, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER7(mut self, v: i32) -> Self {
        self.header_set(39);
        self.values.insert(39, v as u32);
        self
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(47);
        self.values.insert(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(54);
        self.values.insert(54, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(mut self, v: i32) -> Self {
        self.header_set(56);
        self.values.insert(56, v as u32);
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(60);
        self.values.insert(60, v as u32);
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(61);
        self.values.insert(61, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(62);
        self.values.insert(62, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(64);
        self.values.insert(64, v as u32);
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(65);
        self.values.insert(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(66);
        self.values.insert(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(67);
        self.values.insert(67, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(68);
        self.values.insert(68, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(69);
        self.values.insert(69, v as u32);
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(70);
        self.values.insert(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(71);
        self.values.insert(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(72);
        self.values.insert(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(73);
        self.values.insert(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(74);
        self.values.insert(74, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(75);
        self.values.insert(75, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(76);
        self.values.insert(76, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(77);
        self.values.insert(77, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(78);
        self.values.insert(78, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(79);
        self.values.insert(79, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(80);
        self.values.insert(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(81);
        self.values.insert(81, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(82);
        self.values.insert(82, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(83);
        self.values.insert(83, v as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(84);
        self.values.insert(84, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(85);
        self.values.insert(85, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(86);
        self.values.insert(86, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(87);
        self.values.insert(87, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(88);
        self.values.insert(88, v as u32);
        self
    }

    pub fn set_unit_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(89);
        self.values.insert(89, v as u32);
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(90);
        self.values.insert(90, v as u32);
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(91);
        self.values.insert(91, v as u32);
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(92);
        self.values.insert(92, v as u32);
        self
    }

    pub fn set_unit_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(93);
        self.values.insert(93, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(94);
        self.values.insert(94, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(95);
        self.values.insert(95, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(96);
        self.values.insert(96, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(97);
        self.values.insert(97, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(98);
        self.values.insert(98, v as u32);
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(99);
        self.values.insert(99, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(106);
        self.values.insert(106, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(113);
        self.values.insert(113, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(120);
        self.values.insert(120, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(121);
        self.values.insert(121, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122);
        self.values.insert(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(123);
        self.values.insert(123, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(124);
        self.values.insert(124, v);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(125);
        self.values.insert(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(126);
        self.values.insert(126, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(127);
        self.values.insert(127, v);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(128);
        self.values.insert(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(129);
        self.values.insert(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(130);
        self.values.insert(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(131);
        self.values.insert(131, v as u32);
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(138);
        self.values.insert(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(145);
        self.values.insert(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOVERHEIGHT(mut self, v: f32) -> Self {
        self.header_set(146);
        self.values.insert(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdatePlayer {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CRITTER(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23);
        self.values.insert(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn set_unit_POWER6(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn set_unit_POWER7(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(36);
        self.values.insert(36, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER6(mut self, v: i32) -> Self {
        self.header_set(38);
        self.values.insert(38, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER7(mut self, v: i32) -> Self {
        self.header_set(39);
        self.values.insert(39, v as u32);
        self
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(47);
        self.values.insert(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(54);
        self.values.insert(54, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(mut self, v: i32) -> Self {
        self.header_set(56);
        self.values.insert(56, v as u32);
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(60);
        self.values.insert(60, v as u32);
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(61);
        self.values.insert(61, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(62);
        self.values.insert(62, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(64);
        self.values.insert(64, v as u32);
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(65);
        self.values.insert(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(66);
        self.values.insert(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(67);
        self.values.insert(67, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(68);
        self.values.insert(68, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(69);
        self.values.insert(69, v as u32);
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(70);
        self.values.insert(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(71);
        self.values.insert(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(72);
        self.values.insert(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(73);
        self.values.insert(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(74);
        self.values.insert(74, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(75);
        self.values.insert(75, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(76);
        self.values.insert(76, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(77);
        self.values.insert(77, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(78);
        self.values.insert(78, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(79);
        self.values.insert(79, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(80);
        self.values.insert(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(81);
        self.values.insert(81, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(82);
        self.values.insert(82, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(83);
        self.values.insert(83, v as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(84);
        self.values.insert(84, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(85);
        self.values.insert(85, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(86);
        self.values.insert(86, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(87);
        self.values.insert(87, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(88);
        self.values.insert(88, v as u32);
        self
    }

    pub fn set_unit_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(89);
        self.values.insert(89, v as u32);
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(90);
        self.values.insert(90, v as u32);
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(91);
        self.values.insert(91, v as u32);
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(92);
        self.values.insert(92, v as u32);
        self
    }

    pub fn set_unit_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(93);
        self.values.insert(93, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(94);
        self.values.insert(94, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(95);
        self.values.insert(95, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(96);
        self.values.insert(96, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(97);
        self.values.insert(97, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(98);
        self.values.insert(98, v as u32);
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(99);
        self.values.insert(99, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(106);
        self.values.insert(106, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(113);
        self.values.insert(113, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(120);
        self.values.insert(120, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(121);
        self.values.insert(121, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122);
        self.values.insert(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(123);
        self.values.insert(123, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(124);
        self.values.insert(124, v);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(125);
        self.values.insert(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(126);
        self.values.insert(126, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(127);
        self.values.insert(127, v);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(128);
        self.values.insert(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(129);
        self.values.insert(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(130);
        self.values.insert(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(131);
        self.values.insert(131, v as u32);
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(138);
        self.values.insert(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(145);
        self.values.insert(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOVERHEIGHT(mut self, v: f32) -> Self {
        self.header_set(146);
        self.values.insert(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(148);
        self.header_set(149);
        self.values.insert(148, v.guid() as u32);
        self.values.insert(149, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(150);
        self.values.insert(150, v as u32);
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(151);
        self.values.insert(151, v as u32);
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(152);
        self.values.insert(152, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(153);
        self.values.insert(153, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(154);
        self.values.insert(154, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(155);
        self.values.insert(155, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(156);
        self.values.insert(156, v as u32);
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(157);
        self.values.insert(157, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(158);
        self.values.insert(158, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(159);
        self.values.insert(159, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_3(mut self, v: u32) -> Self {
        self.header_set(160);
        self.values.insert(160, v);
        self
    }

    pub fn set_player_QUEST_LOG_1_4(mut self, v: i32) -> Self {
        self.header_set(162);
        self.values.insert(162, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(163);
        self.values.insert(163, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(164);
        self.values.insert(164, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_3(mut self, v: u32) -> Self {
        self.header_set(165);
        self.values.insert(165, v);
        self
    }

    pub fn set_player_QUEST_LOG_2_5(mut self, v: i32) -> Self {
        self.header_set(167);
        self.values.insert(167, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(168);
        self.values.insert(168, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(169);
        self.values.insert(169, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_3(mut self, v: u32) -> Self {
        self.header_set(170);
        self.values.insert(170, v);
        self
    }

    pub fn set_player_QUEST_LOG_3_5(mut self, v: i32) -> Self {
        self.header_set(172);
        self.values.insert(172, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(173);
        self.values.insert(173, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(174);
        self.values.insert(174, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_3(mut self, v: u32) -> Self {
        self.header_set(175);
        self.values.insert(175, v);
        self
    }

    pub fn set_player_QUEST_LOG_4_5(mut self, v: i32) -> Self {
        self.header_set(177);
        self.values.insert(177, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(178);
        self.values.insert(178, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(179);
        self.values.insert(179, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_3(mut self, v: u32) -> Self {
        self.header_set(180);
        self.values.insert(180, v);
        self
    }

    pub fn set_player_QUEST_LOG_5_5(mut self, v: i32) -> Self {
        self.header_set(182);
        self.values.insert(182, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(183);
        self.values.insert(183, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(184);
        self.values.insert(184, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_3(mut self, v: u32) -> Self {
        self.header_set(185);
        self.values.insert(185, v);
        self
    }

    pub fn set_player_QUEST_LOG_6_5(mut self, v: i32) -> Self {
        self.header_set(187);
        self.values.insert(187, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(188);
        self.values.insert(188, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(189);
        self.values.insert(189, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_3(mut self, v: u32) -> Self {
        self.header_set(190);
        self.values.insert(190, v);
        self
    }

    pub fn set_player_QUEST_LOG_7_5(mut self, v: i32) -> Self {
        self.header_set(192);
        self.values.insert(192, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(193);
        self.values.insert(193, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(194);
        self.values.insert(194, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_3(mut self, v: u32) -> Self {
        self.header_set(195);
        self.values.insert(195, v);
        self
    }

    pub fn set_player_QUEST_LOG_8_5(mut self, v: i32) -> Self {
        self.header_set(197);
        self.values.insert(197, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(198);
        self.values.insert(198, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(199);
        self.values.insert(199, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_3(mut self, v: u32) -> Self {
        self.header_set(200);
        self.values.insert(200, v);
        self
    }

    pub fn set_player_QUEST_LOG_9_5(mut self, v: i32) -> Self {
        self.header_set(202);
        self.values.insert(202, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(203);
        self.values.insert(203, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(204);
        self.values.insert(204, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_3(mut self, v: u32) -> Self {
        self.header_set(205);
        self.values.insert(205, v);
        self
    }

    pub fn set_player_QUEST_LOG_10_5(mut self, v: i32) -> Self {
        self.header_set(207);
        self.values.insert(207, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(208);
        self.values.insert(208, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(209);
        self.values.insert(209, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_3(mut self, v: u32) -> Self {
        self.header_set(210);
        self.values.insert(210, v);
        self
    }

    pub fn set_player_QUEST_LOG_11_5(mut self, v: i32) -> Self {
        self.header_set(212);
        self.values.insert(212, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(213);
        self.values.insert(213, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(214);
        self.values.insert(214, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_3(mut self, v: u32) -> Self {
        self.header_set(215);
        self.values.insert(215, v);
        self
    }

    pub fn set_player_QUEST_LOG_12_5(mut self, v: i32) -> Self {
        self.header_set(217);
        self.values.insert(217, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(218);
        self.values.insert(218, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(219);
        self.values.insert(219, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_3(mut self, v: u32) -> Self {
        self.header_set(220);
        self.values.insert(220, v);
        self
    }

    pub fn set_player_QUEST_LOG_13_5(mut self, v: i32) -> Self {
        self.header_set(222);
        self.values.insert(222, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(223);
        self.values.insert(223, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(224);
        self.values.insert(224, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_3(mut self, v: u32) -> Self {
        self.header_set(225);
        self.values.insert(225, v);
        self
    }

    pub fn set_player_QUEST_LOG_14_5(mut self, v: i32) -> Self {
        self.header_set(227);
        self.values.insert(227, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(228);
        self.values.insert(228, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(229);
        self.values.insert(229, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_3(mut self, v: u32) -> Self {
        self.header_set(230);
        self.values.insert(230, v);
        self
    }

    pub fn set_player_QUEST_LOG_15_5(mut self, v: i32) -> Self {
        self.header_set(232);
        self.values.insert(232, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(233);
        self.values.insert(233, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(234);
        self.values.insert(234, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_3(mut self, v: u32) -> Self {
        self.header_set(235);
        self.values.insert(235, v);
        self
    }

    pub fn set_player_QUEST_LOG_16_5(mut self, v: i32) -> Self {
        self.header_set(237);
        self.values.insert(237, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(238);
        self.values.insert(238, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(239);
        self.values.insert(239, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_3(mut self, v: u32) -> Self {
        self.header_set(240);
        self.values.insert(240, v);
        self
    }

    pub fn set_player_QUEST_LOG_17_5(mut self, v: i32) -> Self {
        self.header_set(242);
        self.values.insert(242, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(243);
        self.values.insert(243, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(244);
        self.values.insert(244, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_3(mut self, v: u32) -> Self {
        self.header_set(245);
        self.values.insert(245, v);
        self
    }

    pub fn set_player_QUEST_LOG_18_5(mut self, v: i32) -> Self {
        self.header_set(247);
        self.values.insert(247, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(248);
        self.values.insert(248, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(249);
        self.values.insert(249, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_3(mut self, v: u32) -> Self {
        self.header_set(250);
        self.values.insert(250, v);
        self
    }

    pub fn set_player_QUEST_LOG_19_5(mut self, v: i32) -> Self {
        self.header_set(252);
        self.values.insert(252, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(253);
        self.values.insert(253, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(254);
        self.values.insert(254, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_3(mut self, v: u32) -> Self {
        self.header_set(255);
        self.values.insert(255, v);
        self
    }

    pub fn set_player_QUEST_LOG_20_5(mut self, v: i32) -> Self {
        self.header_set(257);
        self.values.insert(257, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_1(mut self, v: i32) -> Self {
        self.header_set(258);
        self.values.insert(258, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_2(mut self, v: i32) -> Self {
        self.header_set(259);
        self.values.insert(259, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_3(mut self, v: u32) -> Self {
        self.header_set(260);
        self.values.insert(260, v);
        self
    }

    pub fn set_player_QUEST_LOG_21_5(mut self, v: i32) -> Self {
        self.header_set(262);
        self.values.insert(262, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_1(mut self, v: i32) -> Self {
        self.header_set(263);
        self.values.insert(263, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_2(mut self, v: i32) -> Self {
        self.header_set(264);
        self.values.insert(264, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_3(mut self, v: u32) -> Self {
        self.header_set(265);
        self.values.insert(265, v);
        self
    }

    pub fn set_player_QUEST_LOG_22_5(mut self, v: i32) -> Self {
        self.header_set(267);
        self.values.insert(267, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_1(mut self, v: i32) -> Self {
        self.header_set(268);
        self.values.insert(268, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_2(mut self, v: i32) -> Self {
        self.header_set(269);
        self.values.insert(269, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_3(mut self, v: u32) -> Self {
        self.header_set(270);
        self.values.insert(270, v);
        self
    }

    pub fn set_player_QUEST_LOG_23_5(mut self, v: i32) -> Self {
        self.header_set(272);
        self.values.insert(272, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_1(mut self, v: i32) -> Self {
        self.header_set(273);
        self.values.insert(273, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_2(mut self, v: i32) -> Self {
        self.header_set(274);
        self.values.insert(274, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_3(mut self, v: u32) -> Self {
        self.header_set(275);
        self.values.insert(275, v);
        self
    }

    pub fn set_player_QUEST_LOG_24_5(mut self, v: i32) -> Self {
        self.header_set(277);
        self.values.insert(277, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_1(mut self, v: i32) -> Self {
        self.header_set(278);
        self.values.insert(278, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_2(mut self, v: i32) -> Self {
        self.header_set(279);
        self.values.insert(279, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_3(mut self, v: u32) -> Self {
        self.header_set(280);
        self.values.insert(280, v);
        self
    }

    pub fn set_player_QUEST_LOG_25_5(mut self, v: i32) -> Self {
        self.header_set(282);
        self.values.insert(282, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(283);
        self.values.insert(283, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(284);
        self.values.insert(284, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(285);
        self.values.insert(285, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(286);
        self.values.insert(286, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(287);
        self.values.insert(287, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(288);
        self.values.insert(288, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(289);
        self.values.insert(289, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(290);
        self.values.insert(290, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(291);
        self.values.insert(291, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(292);
        self.values.insert(292, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(293);
        self.values.insert(293, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(294);
        self.values.insert(294, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(295);
        self.values.insert(295, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(296);
        self.values.insert(296, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(297);
        self.values.insert(297, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(298);
        self.values.insert(298, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(299);
        self.values.insert(299, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(300);
        self.values.insert(300, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(301);
        self.values.insert(301, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(302);
        self.values.insert(302, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(303);
        self.values.insert(303, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(304);
        self.values.insert(304, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(305);
        self.values.insert(305, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(306);
        self.values.insert(306, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(307);
        self.values.insert(307, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(308);
        self.values.insert(308, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(309);
        self.values.insert(309, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(310);
        self.values.insert(310, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(311);
        self.values.insert(311, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(312);
        self.values.insert(312, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(313);
        self.values.insert(313, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(314);
        self.values.insert(314, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(315);
        self.values.insert(315, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(316);
        self.values.insert(316, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(317);
        self.values.insert(317, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(318);
        self.values.insert(318, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(319);
        self.values.insert(319, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_ENCHANTMENT(mut self, v: u32) -> Self {
        self.header_set(320);
        self.values.insert(320, v);
        self
    }

    pub fn set_player_CHOSEN_TITLE(mut self, v: i32) -> Self {
        self.header_set(321);
        self.values.insert(321, v as u32);
        self
    }

    pub fn set_player_FAKE_INEBRIATION(mut self, v: i32) -> Self {
        self.header_set(322);
        self.values.insert(322, v as u32);
        self
    }

    pub fn set_player_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(324);
        self.header_set(325);
        self.values.insert(324, v.guid() as u32);
        self.values.insert(325, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(370);
        self.header_set(371);
        self.values.insert(370, v.guid() as u32);
        self.values.insert(371, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(402);
        self.header_set(403);
        self.values.insert(402, v.guid() as u32);
        self.values.insert(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(458);
        self.header_set(459);
        self.values.insert(458, v.guid() as u32);
        self.values.insert(459, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(472);
        self.header_set(473);
        self.values.insert(472, v.guid() as u32);
        self.values.insert(473, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(496);
        self.header_set(497);
        self.values.insert(496, v.guid() as u32);
        self.values.insert(497, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_CURRENCYTOKEN_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(560);
        self.header_set(561);
        self.values.insert(560, v.guid() as u32);
        self.values.insert(561, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(624);
        self.header_set(625);
        self.values.insert(624, v.guid() as u32);
        self.values.insert(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES(mut self, v: Guid) -> Self {
        self.header_set(626);
        self.header_set(627);
        self.values.insert(626, v.guid() as u32);
        self.values.insert(627, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES1(mut self, v: Guid) -> Self {
        self.header_set(628);
        self.header_set(629);
        self.values.insert(628, v.guid() as u32);
        self.values.insert(629, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES2(mut self, v: Guid) -> Self {
        self.header_set(630);
        self.header_set(631);
        self.values.insert(630, v.guid() as u32);
        self.values.insert(631, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_CURRENCIES(mut self, v: Guid) -> Self {
        self.header_set(632);
        self.header_set(633);
        self.values.insert(632, v.guid() as u32);
        self.values.insert(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(634);
        self.values.insert(634, v as u32);
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(635);
        self.values.insert(635, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_1(mut self, v: u32) -> Self {
        self.header_set(636);
        self.values.insert(636, v);
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1020);
        self.values.insert(1020, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1021);
        self.values.insert(1021, v as u32);
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1022);
        self.values.insert(1022, v as u32);
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1023);
        self.values.insert(1023, v as u32);
        self
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1024);
        self.values.insert(1024, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1025);
        self.values.insert(1025, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1026);
        self.values.insert(1026, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1027);
        self.values.insert(1027, v as u32);
        self
    }

    pub fn set_player_OFFHAND_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1028);
        self.values.insert(1028, v as u32);
        self
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1029);
        self.values.insert(1029, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1030);
        self.values.insert(1030, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1031);
        self.values.insert(1031, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(mut self, v: f32) -> Self {
        self.header_set(1032);
        self.values.insert(1032, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SHIELD_BLOCK(mut self, v: i32) -> Self {
        self.header_set(1039);
        self.values.insert(1039, v as u32);
        self
    }

    pub fn set_player_SHIELD_BLOCK_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1040);
        self.values.insert(1040, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1041);
        self.values.insert(1041, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1169);
        self.values.insert(1169, v as u32);
        self
    }

    pub fn set_player_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1170);
        self.values.insert(1170, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1171);
        self.values.insert(1171, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1178);
        self.values.insert(1178, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1185);
        self.values.insert(1185, v as u32);
        self
    }

    pub fn set_player_MOD_HEALING_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1192);
        self.values.insert(1192, v as u32);
        self
    }

    pub fn set_player_MOD_HEALING_PCT(mut self, v: f32) -> Self {
        self.header_set(1193);
        self.values.insert(1193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_HEALING_DONE_PCT(mut self, v: f32) -> Self {
        self.header_set(1194);
        self.values.insert(1194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_TARGET_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1195);
        self.values.insert(1195, v as u32);
        self
    }

    pub fn set_player_MOD_TARGET_PHYSICAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1196);
        self.values.insert(1196, v as u32);
        self
    }

    pub fn set_player_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1197);
        self.values.insert(1197, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1198);
        self.values.insert(1198, v as u32);
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1199);
        self.values.insert(1199, v as u32);
        self
    }

    pub fn set_player_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1200);
        self.values.insert(1200, v as u32);
        self
    }

    pub fn set_player_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1201);
        self.values.insert(1201, v as u32);
        self
    }

    pub fn set_player_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1213);
        self.values.insert(1213, v as u32);
        self
    }

    pub fn set_player_KILLS(mut self, v: u32) -> Self {
        self.header_set(1225);
        self.values.insert(1225, v);
        self
    }

    pub fn set_player_TODAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1226);
        self.values.insert(1226, v as u32);
        self
    }

    pub fn set_player_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1227);
        self.values.insert(1227, v as u32);
        self
    }

    pub fn set_player_LIFETIME_HONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1228);
        self.values.insert(1228, v as u32);
        self
    }

    pub fn set_player_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1229);
        self.values.insert(1229, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1230);
        self.values.insert(1230, v as u32);
        self
    }

    pub fn set_player_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1231);
        self.values.insert(1231, v as u32);
        self
    }

    pub fn set_player_ARENA_TEAM_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(1256);
        self.values.insert(1256, v as u32);
        self
    }

    pub fn set_player_HONOR_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1277);
        self.values.insert(1277, v as u32);
        self
    }

    pub fn set_player_ARENA_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1278);
        self.values.insert(1278, v as u32);
        self
    }

    pub fn set_player_MAX_LEVEL(mut self, v: i32) -> Self {
        self.header_set(1279);
        self.values.insert(1279, v as u32);
        self
    }

    pub fn set_player_DAILY_QUESTS_1(mut self, v: i32) -> Self {
        self.header_set(1280);
        self.values.insert(1280, v as u32);
        self
    }

    pub fn set_player_RUNE_REGEN_1(mut self, v: f32) -> Self {
        self.header_set(1305);
        self.values.insert(1305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NO_REAGENT_COST_1(mut self, v: i32) -> Self {
        self.header_set(1309);
        self.values.insert(1309, v as u32);
        self
    }

    pub fn set_player_GLYPH_SLOTS_1(mut self, v: i32) -> Self {
        self.header_set(1312);
        self.values.insert(1312, v as u32);
        self
    }

    pub fn set_player_GLYPHS_1(mut self, v: i32) -> Self {
        self.header_set(1318);
        self.values.insert(1318, v as u32);
        self
    }

    pub fn set_player_GLYPHS_ENABLED(mut self, v: i32) -> Self {
        self.header_set(1324);
        self.values.insert(1324, v as u32);
        self
    }

    pub fn set_player_PET_SPELL_POWER(mut self, v: i32) -> Self {
        self.header_set(1325);
        self.values.insert(1325, v as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateGameObject {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8);
        self.values.insert(8, v as u32);
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn set_gameobject_PARENTROTATION(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_DYNAMIC(mut self, v: u32) -> Self {
        self.header_set(14);
        self.values.insert(14, v);
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn set_gameobject_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(17);
        self.values.insert(17, u32::from_le_bytes([a, b, c, d]));
        self
    }

}

impl UpdateDynamicObject {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_CASTER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8);
        self.values.insert(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_dynamicobject_SPELLID(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_CASTTIME(mut self, v: i32) -> Self {
        self.header_set(11);
        self.values.insert(11, v as u32);
        self
    }

}

impl UpdateCorpse {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_PARTY(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(10);
        self.values.insert(10, v as u32);
        self
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(11);
        self.values.insert(11, v as u32);
        self
    }

    pub fn set_corpse_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(30);
        self.values.insert(30, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(31);
        self.values.insert(31, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_GUILD(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

}
