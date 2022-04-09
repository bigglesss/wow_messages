use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_move_root.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_move_root.wowm#L3):
/// ```text
/// smsg SMSG_FORCE_MOVE_ROOT = 0xE8 {
///     Guid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_FORCE_MOVE_ROOT {
    pub guid: Guid,
    pub counter: u32,
}

impl WorldServerMessageWrite for SMSG_FORCE_MOVE_ROOT {
    const OPCODE: u16 = 0xe8;

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
impl WorldMessageBody for SMSG_FORCE_MOVE_ROOT {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_FORCE_MOVE_ROOT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_FORCE_MOVE_ROOT {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // counter: u32
    }
}
