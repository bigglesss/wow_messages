use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Sent after a successful [`CMSG_AUTH_SESSION`](crate::world::vanilla::CMSG_AUTH_SESSION) and [`SMSG_AUTH_RESPONSE`](crate::world::vanilla::SMSG_AUTH_RESPONSE), or after failing to login with [`SMSG_CHARACTER_LOGIN_FAILED`](crate::world::vanilla::SMSG_CHARACTER_LOGIN_FAILED).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_enum.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_enum.wowm#L1):
/// ```text
/// cmsg CMSG_CHAR_ENUM = 0x0037 {
/// }
/// ```
pub struct CMSG_CHAR_ENUM {
}

impl ClientMessage for CMSG_CHAR_ENUM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x0037;

    fn client_size(&self) -> u16 {
        6
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_CHAR_ENUM;
    use super::*;
    use super::super::*;
    use crate::world::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x37, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_enum.wowm` line 6.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_ENUM0() {
        let expected = CMSG_CHAR_ENUM {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_enum.wowm` line 6.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_CHAR_ENUM0() {
        let expected = CMSG_CHAR_ENUM {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_enum.wowm` line 6.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_CHAR_ENUM0() {
        let expected = CMSG_CHAR_ENUM {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
