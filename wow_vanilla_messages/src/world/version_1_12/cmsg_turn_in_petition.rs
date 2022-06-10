use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm#L3):
/// ```text
/// cmsg CMSG_TURN_IN_PETITION = 0x01C4 {
///     Guid petition_guid;
/// }
/// ```
pub struct CMSG_TURN_IN_PETITION {
    pub petition_guid: Guid,
}

impl ClientMessage for CMSG_TURN_IN_PETITION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        Ok(Self {
            petition_guid,
        })
    }

}

