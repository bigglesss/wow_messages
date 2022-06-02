use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::TimerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm#L5):
/// ```text
/// smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
///     TimerType timer;
///     u8 is_frozen;
/// }
/// ```
/// # Comment
///
/// According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with SMSG_START_MIRROR_TIMER to avoid lua errors.
pub struct SMSG_PAUSE_MIRROR_TIMER {
    pub timer: TimerType,
    pub is_frozen: u8,
}

impl ServerMessage for SMSG_PAUSE_MIRROR_TIMER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01da;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

}
