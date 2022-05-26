use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_FISH_ESCAPED {
}

impl SMSG_FISH_ESCAPED {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl ServerMessage for SMSG_FISH_ESCAPED {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x01c9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

}

