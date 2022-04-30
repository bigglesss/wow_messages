use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
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
pub struct MSG_MOVE_WORLDPORT_ACK {
}

impl ClientMessageWrite for MSG_MOVE_WORLDPORT_ACK {}

impl ServerMessageWrite for MSG_MOVE_WORLDPORT_ACK {}

impl MessageBody for MSG_MOVE_WORLDPORT_ACK {
    const OPCODE: u16 = 0x00dc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

impl ConstantSized for MSG_MOVE_WORLDPORT_ACK {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_MOVE_WORLDPORT_ACK {
    fn maximum_possible_size() -> usize {
        0
    }
}

