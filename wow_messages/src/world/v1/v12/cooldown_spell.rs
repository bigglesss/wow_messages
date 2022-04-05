use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4745`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4745):
/// ```text
/// struct CooldownSpell {
///     u16 spell_id;
///     u16 item_id;
///     u16 spell_category;
///     u32 cooldown_in_msecs;
///     u32 category_cooldown_in_msecs;
/// }
/// ```
pub struct CooldownSpell {
    pub spell_id: u16,
    pub item_id: u16,
    pub spell_category: u16,
    pub cooldown_in_msecs: u32,
    pub category_cooldown_in_msecs: u32,
}

impl ReadableAndWritable for CooldownSpell {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // item_id: u16
        let item_id = crate::util::read_u16_le(r)?;

        // spell_category: u16
        let spell_category = crate::util::read_u16_le(r)?;

        // cooldown_in_msecs: u32
        let cooldown_in_msecs = crate::util::read_u32_le(r)?;

        // category_cooldown_in_msecs: u32
        let category_cooldown_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell_id,
            item_id,
            spell_category,
            cooldown_in_msecs,
            category_cooldown_in_msecs,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // item_id: u16
        w.write_all(&self.item_id.to_le_bytes())?;

        // spell_category: u16
        w.write_all(&self.spell_category.to_le_bytes())?;

        // cooldown_in_msecs: u32
        w.write_all(&self.cooldown_in_msecs.to_le_bytes())?;

        // category_cooldown_in_msecs: u32
        w.write_all(&self.category_cooldown_in_msecs.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CooldownSpell {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CooldownSpell {
    fn maximum_possible_size() -> usize {
        2 // spell_id: u16
        + 2 // item_id: u16
        + 2 // spell_category: u16
        + 4 // cooldown_in_msecs: u32
        + 4 // category_cooldown_in_msecs: u32
    }
}

