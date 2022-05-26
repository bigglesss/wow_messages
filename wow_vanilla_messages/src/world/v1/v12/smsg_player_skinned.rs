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
pub struct SMSG_PLAYER_SKINNED {
}

impl SMSG_PLAYER_SKINNED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 0], std::io::Error> {
        let mut array_w = [0u8; 0];
        let mut w = array_w.as_mut_slice();
        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PLAYER_SKINNED {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x02bc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

}

