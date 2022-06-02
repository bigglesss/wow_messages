use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/cmsg_toggle_pvp.wowm#L5):
/// ```text
/// cmsg CMSG_TOGGLE_PVP = 0x0253 {
///     optional set {
///         u8 enable_pvp;
///     }
/// }
/// ```
/// # Comment
///
/// vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status
pub struct CMSG_TOGGLE_PVP {
    pub set: Option<CMSG_TOGGLE_PVP_set>,
}

impl ClientMessage for CMSG_TOGGLE_PVP {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: u8
            w.write_all(&v.enable_pvp.to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x0253;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // enable_pvp: u8
            let enable_pvp = crate::util::read_u8_le(r)?;

            Some(CMSG_TOGGLE_PVP_set {
                enable_pvp,
            })
        } else {
            None
        };

        Ok(Self {
            set,
        })
    }

}

impl CMSG_TOGGLE_PVP {
    pub(crate) fn size(&self) -> usize {
        if let Some(set) = &self.set {
            1 // enable_pvp: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_TOGGLE_PVP_set {
    pub enable_pvp: u8,
}

impl CMSG_TOGGLE_PVP_set {
    pub(crate) fn size(&self) -> usize {
        1 // enable_pvp: u8
    }

}
