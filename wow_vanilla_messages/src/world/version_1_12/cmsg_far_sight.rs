use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::FarSightOperation;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_far_sight.wowm#L8):
/// ```text
/// cmsg CMSG_FAR_SIGHT = 0x027A {
///     FarSightOperation operation;
/// }
/// ```
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl ClientMessage for CMSG_FAR_SIGHT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // operation: FarSightOperation
        w.write_all(&(self.operation.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x027a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // operation: FarSightOperation
        let operation: FarSightOperation = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            operation,
        })
    }

}

