use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_taxinode_status_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_taxinode_status_query.wowm#L3):
/// ```text
/// cmsg CMSG_TAXINODE_STATUS_QUERY = 0x01AA {
///     Guid guid;
/// }
/// ```
pub struct CMSG_TAXINODE_STATUS_QUERY {
    pub guid: Guid,
}

impl ClientMessage for CMSG_TAXINODE_STATUS_QUERY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01aa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}

