use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SET_FACTION_INACTIVE {
    pub reputation_list_id: u32,
    pub inactive: u8,
}

impl ClientMessageWrite for CMSG_SET_FACTION_INACTIVE {}

impl MessageBody for CMSG_SET_FACTION_INACTIVE {
    const OPCODE: u16 = 0x0317;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // inactive: u8
        let inactive = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            inactive,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // inactive: u8
        w.write_all(&self.inactive.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SET_FACTION_INACTIVE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SET_FACTION_INACTIVE {
    fn maximum_possible_size() -> usize {
        4 // reputation_list_id: u32
        + 1 // inactive: u8
    }
}

