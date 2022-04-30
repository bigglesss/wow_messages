use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
    pub owner_guid: Guid,
    pub petition_guid: Guid,
    pub amount_of_signatures: u8,
}

impl ServerMessageWrite for SMSG_PETITION_SHOW_SIGNATURES {}

impl MessageBody for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u16 = 0x01bf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            amount_of_signatures,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: Guid
        self.item_guid.write(w)?;

        // owner_guid: Guid
        self.owner_guid.write(w)?;

        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // amount_of_signatures: u8
        w.write_all(&self.amount_of_signatures.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PETITION_SHOW_SIGNATURES {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PETITION_SHOW_SIGNATURES {
    fn maximum_possible_size() -> usize {
        8 // item_guid: Guid
        + 8 // owner_guid: Guid
        + 8 // petition_guid: Guid
        + 1 // amount_of_signatures: u8
    }
}

