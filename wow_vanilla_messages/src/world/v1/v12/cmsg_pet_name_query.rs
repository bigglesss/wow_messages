use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm#L3):
/// ```text
/// cmsg CMSG_PET_NAME_QUERY = 0x52 {
///     u32 pet_number;
///     u64 guid;
/// }
/// ```
pub struct CMSG_PET_NAME_QUERY {
    pub pet_number: u32,
    pub guid: u64,
}

impl WorldClientMessageWrite for CMSG_PET_NAME_QUERY {
    const OPCODE: u32 = 0x52;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_PET_NAME_QUERY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            pet_number,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_NAME_QUERY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PET_NAME_QUERY {
    fn maximum_possible_size() -> usize {
        4 // pet_number: u32
        + 8 // guid: u64
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMSG_PET_NAME_QUERY;
    use crate::ConstantSized;
    use crate::world::v1::v12::opcodes::WorldClientOpcodeMessage;
    use crate::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[test]
    fn CMSG_PET_NAME_QUERY0() {
        let raw: Vec<u8> = vec![ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: 0xFACADEDEADBEEF,
        };

        let header_size = 2 + 4;
        let t = WorldClientOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);

        assert_eq!(CMSG_PET_NAME_QUERY::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}