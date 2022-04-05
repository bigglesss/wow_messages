use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_areatrigger.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_areatrigger.wowm#L3):
/// ```text
/// cmsg CMSG_AREATRIGGER = 0xB4 {
///     u32 trigger_id;
/// }
/// ```
pub struct CMSG_AREATRIGGER {
    pub trigger_id: u32,
}

impl WorldClientMessageWrite for CMSG_AREATRIGGER {
    const OPCODE: u32 = 0xb4;

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
impl WorldMessageBody for CMSG_AREATRIGGER {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // trigger_id: u32
        let trigger_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            trigger_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // trigger_id: u32
        w.write_all(&self.trigger_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AREATRIGGER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_AREATRIGGER {
    fn maximum_possible_size() -> usize {
        4 // trigger_id: u32
    }
}

