use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
}

impl ClientMessage for CMSG_GUILD_RANK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0231;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // rank_id: u32
        let rank_id = crate::util::read_u32_le(r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // rank_name: CString
        let rank_name = crate::util::read_c_string_to_vec(r)?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

}

impl CMSG_GUILD_RANK {
    pub(crate) fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString
    }
}

