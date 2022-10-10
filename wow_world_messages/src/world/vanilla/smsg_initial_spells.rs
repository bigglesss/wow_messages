use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::CooldownSpell;
use crate::world::vanilla::InitialSpell;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L23):
/// ```text
/// smsg SMSG_INITIAL_SPELLS = 0x012A {
///     u8 unknown1;
///     u16 spell_count;
///     InitialSpell[spell_count] initial_spells;
///     u16 cooldown_count;
///     CooldownSpell[cooldown_count] cooldowns;
/// }
/// ```
pub struct SMSG_INITIAL_SPELLS {
    /// cmangos/mangoszero: sets to 0
    ///
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl crate::Message for SMSG_INITIAL_SPELLS {
    const OPCODE: u32 = 0x012a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write_into_vec(w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // spell_count: u16
        let spell_count = crate::util::read_u16_le(r)?;

        // initial_spells: InitialSpell[spell_count]
        let mut initial_spells = Vec::with_capacity(spell_count as usize);
        for i in 0..spell_count {
            let o = InitialSpell::read(r)?;
            initial_spells.push(o);
        }

        // cooldown_count: u16
        let cooldown_count = crate::util::read_u16_le(r)?;

        // cooldowns: CooldownSpell[cooldown_count]
        let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
        for i in 0..cooldown_count {
            let o = CooldownSpell::read(r)?;
            cooldowns.push(o);
        }

        Ok(Self {
            unknown1,
            initial_spells,
            cooldowns,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_INITIAL_SPELLS {}

impl SMSG_INITIAL_SPELLS {
    pub(crate) fn size(&self) -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.len() * 4 // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.len() * 14 // cooldowns: CooldownSpell[cooldown_count]
    }
}

