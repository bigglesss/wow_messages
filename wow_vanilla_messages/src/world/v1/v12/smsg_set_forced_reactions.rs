use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ForcedReaction;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L12):
/// ```text
/// smsg SMSG_SET_FORCED_REACTIONS = 0x2A5 {
///     u32 amount_of_reactions;
///     ForcedReaction[amount_of_reactions] reactions;
/// }
/// ```
pub struct SMSG_SET_FORCED_REACTIONS {
    pub amount_of_reactions: u32,
    pub reactions: Vec<ForcedReaction>,
}

impl WorldServerMessageWrite for SMSG_SET_FORCED_REACTIONS {
    const OPCODE: u16 = 0x2a5;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SET_FORCED_REACTIONS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
        for i in 0..amount_of_reactions {
            reactions.push(ForcedReaction::read(r)?);
        }

        Ok(Self {
            amount_of_reactions,
            reactions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SET_FORCED_REACTIONS {
    fn size(&self) -> usize {
        4 // amount_of_reactions: u32
        + self.reactions.iter().fold(0, |acc, x| acc + ForcedReaction::size()) // reactions: ForcedReaction[amount_of_reactions]
    }
}

impl MaximumPossibleSized for SMSG_SET_FORCED_REACTIONS {
    fn maximum_possible_size() -> usize {
        4 // amount_of_reactions: u32
        + 4294967295 * ForcedReaction::maximum_possible_size() // reactions: ForcedReaction[amount_of_reactions]
    }
}
