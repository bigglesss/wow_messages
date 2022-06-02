use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm#L5):
/// ```text
/// cmsg CMSG_NEXT_CINEMATIC_CAMERA = 0x00FB {
/// }
/// ```
pub struct CMSG_NEXT_CINEMATIC_CAMERA {
}

impl ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x00fb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        Ok(Self {
        })
    }

}

