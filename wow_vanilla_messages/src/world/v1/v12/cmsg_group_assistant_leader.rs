use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_GROUP_ASSISTANT_LEADER {
    pub guid: Guid,
    pub set_assistant: u8,
}

impl ClientMessage for CMSG_GROUP_ASSISTANT_LEADER {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // set_assistant: u8
        w.write_all(&self.set_assistant.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x028f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // set_assistant: u8
        let set_assistant = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            set_assistant,
        })
    }

}

