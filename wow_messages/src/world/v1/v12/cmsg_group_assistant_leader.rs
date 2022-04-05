use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L67):
/// ```text
/// cmsg CMSG_GROUP_ASSISTANT_LEADER = 0x28F {
///     u64 guid;
///     u8 set_assistant;
/// }
/// ```
pub struct CMSG_GROUP_ASSISTANT_LEADER {
    pub guid: u64,
    pub set_assistant: u8,
}

impl WorldClientMessageWrite for CMSG_GROUP_ASSISTANT_LEADER {
    const OPCODE: u32 = 0x28f;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_GROUP_ASSISTANT_LEADER {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // set_assistant: u8
        let set_assistant = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            set_assistant,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // set_assistant: u8
        w.write_all(&self.set_assistant.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_GROUP_ASSISTANT_LEADER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_GROUP_ASSISTANT_LEADER {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 1 // set_assistant: u8
    }
}

