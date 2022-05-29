use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::WorldState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_UPDATE_WORLD_STATE {
    pub state: WorldState,
}

impl ServerMessage for SMSG_UPDATE_WORLD_STATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // state: WorldState
        self.state.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // state: WorldState
        let state = WorldState::read(r)?;

        Ok(Self {
            state,
        })
    }

}

