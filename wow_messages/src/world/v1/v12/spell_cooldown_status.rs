use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/5needs_endless_array/cmsg_auth_session.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/5needs_endless_array/cmsg_auth_session.wowm#L15):
/// ```text
/// struct SpellCooldownStatus {
///     u32 spell_id;
///     u32 cooldown_time_in_msecs;
/// }
/// ```
pub struct SpellCooldownStatus {
    pub spell_id: u32,
    pub cooldown_time_in_msecs: u32,
}

impl ReadableAndWritable for SpellCooldownStatus {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell_id,
            cooldown_time_in_msecs,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // cooldown_time_in_msecs: u32
        w.write_all(&self.cooldown_time_in_msecs.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for SpellCooldownStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellCooldownStatus {
    fn maximum_possible_size() -> usize {
        4 // spell_id: u32
        + 4 // cooldown_time_in_msecs: u32
    }
}

