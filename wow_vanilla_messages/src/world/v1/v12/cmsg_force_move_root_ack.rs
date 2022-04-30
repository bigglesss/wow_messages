use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_FORCE_MOVE_ROOT_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub movement_info: MovementInfo,
}

impl ClientMessageWrite for CMSG_FORCE_MOVE_ROOT_ACK {
    const OPCODE: u16 = 0xe9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_FORCE_MOVE_ROOT_ACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            movement_counter,
            movement_info,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_FORCE_MOVE_ROOT_ACK {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

impl MaximumPossibleSized for CMSG_FORCE_MOVE_ROOT_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
    }
}

