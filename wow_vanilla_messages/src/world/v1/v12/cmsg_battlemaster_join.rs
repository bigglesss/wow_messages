use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::Map;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_BATTLEMASTER_JOIN {
    pub guid: Guid,
    pub map: Map,
    pub instance_id: u32,
    pub join_as_group: u8,
}

impl ClientMessage for CMSG_BATTLEMASTER_JOIN {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: u8
        w.write_all(&self.join_as_group.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02ee;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        17
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        // join_as_group: u8
        let join_as_group = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            map,
            instance_id,
            join_as_group,
        })
    }

}

