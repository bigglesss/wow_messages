use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Area;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_EXPLORATION_EXPERIENCE {
    pub area: Area,
    pub experience: u32,
}

impl SMSG_EXPLORATION_EXPERIENCE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_EXPLORATION_EXPERIENCE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // experience: u32
        w.write_all(&self.experience.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // experience: u32
        let experience = crate::util::read_u32_le(r)?;

        Ok(Self {
            area,
            experience,
        })
    }

}

