use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_player_logout.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_player_logout.wowm#L3):
/// ```text
/// cmsg CMSG_PLAYER_LOGOUT = 0x004A {
/// }
/// ```
pub struct CMSG_PLAYER_LOGOUT {
}

impl ClientMessage for CMSG_PLAYER_LOGOUT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x004a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        0
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        Ok(Self {
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_PLAYER_LOGOUT;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x4A, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_player_logout.wowm` line 5.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PLAYER_LOGOUT0() {
        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_player_logout.wowm` line 5.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PLAYER_LOGOUT0() {
        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_player_logout.wowm` line 5.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PLAYER_LOGOUT0() {
        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
