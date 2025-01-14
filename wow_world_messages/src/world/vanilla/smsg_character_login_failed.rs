use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::WorldResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Response if [`CMSG_PLAYER_LOGIN`](crate::world::vanilla::CMSG_PLAYER_LOGIN) fails. If successful it should instead be [`SMSG_LOGIN_VERIFY_WORLD`](crate::world::vanilla::SMSG_LOGIN_VERIFY_WORLD).
///
/// Client seems to always send a [`CMSG_CANCEL_TRADE`](crate::world::vanilla::CMSG_CANCEL_TRADE) after receiving this message, for unknown reasons.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm#L1):
/// ```text
/// smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

impl crate::Message for SMSG_CHARACTER_LOGIN_FAILED {
    const OPCODE: u32 = 0x0041;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CHARACTER_LOGIN_FAILED {}

#[cfg(test)]
mod test {
    use super::SMSG_CHARACTER_LOGIN_FAILED;
    use crate::world::vanilla::WorldResult;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x41, 0x00, 0x41, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CharLoginFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CharLoginFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CharLoginFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
