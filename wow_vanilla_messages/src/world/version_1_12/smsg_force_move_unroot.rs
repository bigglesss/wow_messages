use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm#L3):
/// ```text
/// smsg SMSG_FORCE_MOVE_UNROOT = 0x00EA {
///     Guid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_FORCE_MOVE_UNROOT {
    pub guid: Guid,
    pub counter: u32,
}

impl ServerMessage for SMSG_FORCE_MOVE_UNROOT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00ea;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}
