use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Area;
use crate::world::v1::v12::Map;
use crate::world::v1::v12::WorldState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub states: Vec<WorldState>,
}

impl ServerMessage for SMSG_INIT_WORLD_STATES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02c2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(r)?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::read(r)?);
        }

        Ok(Self {
            map,
            area,
            states,
        })
    }

}

impl SMSG_INIT_WORLD_STATES {
    pub fn size(&self) -> usize {
        0
        + 4 // map: Map
        + 4 // area: Area
        + 2 // amount_of_states: u16
        + self.states.len() * 8 // states: WorldState[amount_of_states]
    }
}

