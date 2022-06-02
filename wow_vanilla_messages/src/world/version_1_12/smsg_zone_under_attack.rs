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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L3):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x0254 {
///     u32 zone_id;
/// }
/// ```
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: u32,
}

impl ServerMessage for SMSG_ZONE_UNDER_ATTACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0254;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            zone_id,
        })
    }

}

