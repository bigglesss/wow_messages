use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COUNTDOWN = 0x2B7 {
///     u32 time_in_seconds;
/// }
/// ```
pub struct SMSG_DUEL_COUNTDOWN {
    pub time_in_seconds: u32,
}

impl WorldServerMessageWrite for SMSG_DUEL_COUNTDOWN {
    const OPCODE: u16 = 0x2b7;

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
impl WorldMessageBody for SMSG_DUEL_COUNTDOWN {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // time_in_seconds: u32
        let time_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            time_in_seconds,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // time_in_seconds: u32
        w.write_all(&self.time_in_seconds.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_DUEL_COUNTDOWN {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_DUEL_COUNTDOWN {
    fn maximum_possible_size() -> usize {
        4 // time_in_seconds: u32
    }
}
