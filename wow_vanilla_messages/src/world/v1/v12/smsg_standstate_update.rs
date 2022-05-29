use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::UnitStandState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_STANDSTATE_UPDATE {
    pub state: UnitStandState,
}

impl ServerMessage for SMSG_STANDSTATE_UPDATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // state: UnitStandState
        w.write_all(&(self.state.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x029d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // state: UnitStandState
        let state: UnitStandState = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            state,
        })
    }

}

