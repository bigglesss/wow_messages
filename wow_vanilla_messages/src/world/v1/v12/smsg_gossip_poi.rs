use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GOSSIP_POI {
    pub flags: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub icon: u32,
    pub data: u32,
    pub location_name: String,
}

impl ServerMessage for SMSG_GOSSIP_POI {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // icon: u32
        w.write_all(&self.icon.to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // location_name: CString
        w.write_all(self.location_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0224;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // icon: u32
        let icon = crate::util::read_u32_le(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // location_name: CString
        let location_name = crate::util::read_c_string_to_vec(r)?;
        let location_name = String::from_utf8(location_name)?;

        Ok(Self {
            flags,
            position_x,
            position_y,
            icon,
            data,
            location_name,
        })
    }

}

impl SMSG_GOSSIP_POI {
    pub(crate) fn size(&self) -> usize {
        4 // flags: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // icon: u32
        + 4 // data: u32
        + self.location_name.len() + 1 // location_name: CString
    }
}

