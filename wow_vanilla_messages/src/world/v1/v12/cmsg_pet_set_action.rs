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
pub struct CMSG_PET_SET_ACTION {
    pub guid: Guid,
    pub position1: u32,
    pub data1: u32,
    pub extra: Option<CMSG_PET_SET_ACTIONextra>,
}

impl ClientMessage for CMSG_PET_SET_ACTION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position1: u32
        w.write_all(&self.position1.to_le_bytes())?;

        // data1: u32
        w.write_all(&self.data1.to_le_bytes())?;

        // optional extra
        if let Some(v) = &self.extra {
            // position2: u32
            w.write_all(&v.position2.to_le_bytes())?;

            // data2: u32
            w.write_all(&v.data2.to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x0174;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // position1: u32
        let position1 = crate::util::read_u32_le(r)?;

        // data1: u32
        let data1 = crate::util::read_u32_le(r)?;

        // optional extra
        let current_size = {
            8 // guid: Guid
            + 4 // position1: u32
            + 4 // data1: u32
        };
        let extra = if current_size < body_size as usize {
            // position2: u32
            let position2 = crate::util::read_u32_le(r)?;

            // data2: u32
            let data2 = crate::util::read_u32_le(r)?;

            Some(CMSG_PET_SET_ACTIONextra {
                position2,
                data2,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            position1,
            data1,
            extra,
        })
    }

}

impl CMSG_PET_SET_ACTION {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
        + if let Some(extra) = &self.extra {
            4 // position2: u32
            + 4 // data2: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_PET_SET_ACTIONextra {
    pub position2: u32,
    pub data2: u32,
}

impl CMSG_PET_SET_ACTIONextra {
    pub(crate) fn size(&self) -> usize {
        4 // position2: u32
        + 4 // data2: u32
    }

}

