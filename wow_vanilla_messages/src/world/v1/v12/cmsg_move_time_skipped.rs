use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
#[derive(Copy)]
pub struct CMSG_MOVE_TIME_SKIPPED {
    pub guid: Guid,
    pub lag: u32,
}

impl ClientMessageWrite for CMSG_MOVE_TIME_SKIPPED {
    const OPCODE: u32 = 0x2ce;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_MOVE_TIME_SKIPPED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // lag: u32
        let lag = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            lag,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // lag: u32
        w.write_all(&self.lag.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_MOVE_TIME_SKIPPED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_MOVE_TIME_SKIPPED {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // lag: u32
    }
}

