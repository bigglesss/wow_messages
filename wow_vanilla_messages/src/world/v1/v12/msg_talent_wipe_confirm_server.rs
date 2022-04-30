use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_TALENT_WIPE_CONFIRM_Server {
    pub wiping_npc: Guid,
    pub cost_in_copper: u32,
}

impl WorldServerMessageWrite for MSG_TALENT_WIPE_CONFIRM_Server {
    const OPCODE: u16 = 0x2aa;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_TALENT_WIPE_CONFIRM_Server {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::read_u32_le(r)?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // wiping_npc: Guid
        self.wiping_npc.write(w)?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for MSG_TALENT_WIPE_CONFIRM_Server {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_TALENT_WIPE_CONFIRM_Server {
    fn maximum_possible_size() -> usize {
        8 // wiping_npc: Guid
        + 4 // cost_in_copper: u32
    }
}

