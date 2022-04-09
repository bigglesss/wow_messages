use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_FAILED = 0x196 {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_FAILED {
    pub quest_id: u32,
}

impl WorldServerMessageWrite for SMSG_QUESTUPDATE_FAILED {
    const OPCODE: u16 = 0x196;

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
impl WorldMessageBody for SMSG_QUESTUPDATE_FAILED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            quest_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTUPDATE_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUESTUPDATE_FAILED {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
    }
}
