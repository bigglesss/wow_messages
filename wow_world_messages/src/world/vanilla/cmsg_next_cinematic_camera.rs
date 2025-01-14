use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Sent by client when cinematic beings.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm#L5):
/// ```text
/// cmsg CMSG_NEXT_CINEMATIC_CAMERA = 0x00FB {
/// }
/// ```
pub struct CMSG_NEXT_CINEMATIC_CAMERA {
}

impl crate::Message for CMSG_NEXT_CINEMATIC_CAMERA {
    const OPCODE: u32 = 0x00fb;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

