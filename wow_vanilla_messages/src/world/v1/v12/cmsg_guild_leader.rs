use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_LEADER {
    pub new_guild_leader_name: String,
}

impl ClientMessage for CMSG_GUILD_LEADER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // new_guild_leader_name: CString
        w.write_all(self.new_guild_leader_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0090;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_guild_leader_name: CString
        let new_guild_leader_name = crate::util::read_c_string_to_vec(r)?;
        let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

        Ok(Self {
            new_guild_leader_name,
        })
    }

}

impl CMSG_GUILD_LEADER {
    pub(crate) fn size(&self) -> usize {
        self.new_guild_leader_name.len() + 1 // new_guild_leader_name: CString
    }
}

