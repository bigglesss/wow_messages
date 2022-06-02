use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_CHOOSE_REWARD = 0x018E {
///     Guid guid;
///     u32 quest_id;
///     u32 reward;
/// }
/// ```
pub struct CMSG_QUESTGIVER_CHOOSE_REWARD {
    pub guid: Guid,
    pub quest_id: u32,
    pub reward: u32,
}

impl ClientMessage for CMSG_QUESTGIVER_CHOOSE_REWARD {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reward: u32
        w.write_all(&self.reward.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x018e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // reward: u32
        let reward = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            quest_id,
            reward,
        })
    }

}

