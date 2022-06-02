use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm#L3):
/// ```text
/// cmsg CMSG_ACTIVATETAXIEXPRESS = 0x0312 {
///     Guid guid;
///     u32 total_cost;
///     u32 node_count;
/// }
/// ```
pub struct CMSG_ACTIVATETAXIEXPRESS {
    pub guid: Guid,
    pub total_cost: u32,
    pub node_count: u32,
}

impl ClientMessage for CMSG_ACTIVATETAXIEXPRESS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // total_cost: u32
        w.write_all(&self.total_cost.to_le_bytes())?;

        // node_count: u32
        w.write_all(&self.node_count.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0312;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // total_cost: u32
        let total_cost = crate::util::read_u32_le(r)?;

        // node_count: u32
        let node_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            total_cost,
            node_count,
        })
    }

}
