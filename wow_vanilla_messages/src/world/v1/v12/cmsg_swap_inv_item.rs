use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SWAP_INV_ITEM {
    pub source_slot: u8,
    pub destination_slot: u8,
}

impl ClientMessage for CMSG_SWAP_INV_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x010d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        2
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_slot,
            destination_slot,
        })
    }

}

