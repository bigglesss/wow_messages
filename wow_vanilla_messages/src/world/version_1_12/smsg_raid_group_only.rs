use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::RaidGroupError;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm#L8):
/// ```text
/// smsg SMSG_RAID_GROUP_ONLY = 0x0286 {
///     u32 homebind_timer;
///     RaidGroupError error;
/// }
/// ```
/// # Comment
///
/// used when player leaves raid group inside instance
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl ServerMessage for SMSG_RAID_GROUP_ONLY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0286;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(r)?;

        // error: RaidGroupError
        let error: RaidGroupError = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            homebind_timer,
            error,
        })
    }

}

