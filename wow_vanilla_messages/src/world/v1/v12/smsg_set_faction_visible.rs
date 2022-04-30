use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SET_FACTION_VISIBLE {
    pub reputation_list_id: u32,
}

impl ServerMessageWrite for SMSG_SET_FACTION_VISIBLE {
    const OPCODE: u16 = 0x123;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_SET_FACTION_VISIBLE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SET_FACTION_VISIBLE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SET_FACTION_VISIBLE {
    fn maximum_possible_size() -> usize {
        4 // reputation_list_id: u32
    }
}

