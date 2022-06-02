use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm#L3):
/// ```text
/// smsg SMSG_INSTANCE_SAVE_CREATED = 0x02CB {
///     u32 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_SAVE_CREATED {
    /// # Comment
    ///
    /// cmangos/vmangos/mangoszero set to 0
    pub unknown: u32,
}

impl ServerMessage for SMSG_INSTANCE_SAVE_CREATED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02cb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown,
        })
    }

}

