use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PetitionResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L12):
/// ```text
/// smsg SMSG_PETITION_SIGN_RESULTS = 0x01C1 {
///     Guid petition_guid;
///     Guid owner_guid;
///     PetitionResult result;
/// }
/// ```
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition_guid: Guid,
    pub owner_guid: Guid,
    pub result: PetitionResult,
}

impl ServerMessage for SMSG_PETITION_SIGN_RESULTS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // result: PetitionResult
        let result: PetitionResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

}

