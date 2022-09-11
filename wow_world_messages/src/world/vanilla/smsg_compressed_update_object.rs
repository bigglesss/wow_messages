use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_compressed_update_object.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_compressed_update_object.wowm#L1):
/// ```text
/// smsg SMSG_COMPRESSED_UPDATE_OBJECT = 0x01F6 {
///     unimplemented
/// }
/// ```
pub struct SMSG_COMPRESSED_UPDATE_OBJECT {
    pub unimplemented: Vec<u8>,
}

impl crate::Message for SMSG_COMPRESSED_UPDATE_OBJECT {
    const OPCODE: u32 = 0x01f6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unimplemented: u8[-]
        for i in self.unimplemented.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unimplemented: u8[-]
        let mut current_size = {
            0
        };
        let mut unimplemented = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            unimplemented.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unimplemented,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_COMPRESSED_UPDATE_OBJECT {}

impl SMSG_COMPRESSED_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        self.unimplemented.len() * core::mem::size_of::<u8>() // unimplemented: u8[-]
    }
}

