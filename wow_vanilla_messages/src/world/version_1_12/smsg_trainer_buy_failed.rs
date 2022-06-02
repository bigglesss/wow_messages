use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::TrainingFailureReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm#L18):
/// ```text
/// smsg SMSG_TRAINER_BUY_FAILED = 0x01B4 {
///     Guid guid;
///     u32 id;
///     TrainingFailureReason error;
/// }
/// ```
pub struct SMSG_TRAINER_BUY_FAILED {
    pub guid: Guid,
    pub id: u32,
    pub error: TrainingFailureReason,
}

impl ServerMessage for SMSG_TRAINER_BUY_FAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01b4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // error: TrainingFailureReason
        let error: TrainingFailureReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            id,
            error,
        })
    }

}
