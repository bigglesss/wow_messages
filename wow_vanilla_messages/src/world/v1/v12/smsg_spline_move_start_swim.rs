use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPLINE_MOVE_START_SWIM {
    pub guid: Guid,
}

impl SMSG_SPLINE_MOVE_START_SWIM {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        Ok(())
    }
}

impl ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        Ok(())
    }
    const OPCODE: u16 = 0x030b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}

impl SMSG_SPLINE_MOVE_START_SWIM {
    pub fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
    }
}

