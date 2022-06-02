use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_spirit_healer_activate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_spirit_healer_activate.wowm#L3):
/// ```text
/// cmsg CMSG_SPIRIT_HEALER_ACTIVATE = 0x021C {
///     Guid guid;
/// }
/// ```
pub struct CMSG_SPIRIT_HEALER_ACTIVATE {
    pub guid: Guid,
}

impl ClientMessage for CMSG_SPIRIT_HEALER_ACTIVATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x021c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}

