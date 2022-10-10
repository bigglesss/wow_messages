use std::convert::{TryFrom, TryInto};
use crate::world::shared::addon_info123_3_5::AddonInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Sent after receiving [`SMSG_AUTH_CHALLENGE`](crate::world::vanilla::SMSG_AUTH_CHALLENGE).
///
/// This message is never encrypted.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L10):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x01ED {
///     u32 build;
///     u32 server_id;
///     CString username;
///     u32 client_seed;
///     u8[20] client_proof;
///     u32 decompressed_addon_info_size;
///     AddonInfo[-] addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub build: u32,
    /// This is sent to the client in `CMD_REALM_LIST_Server`.
    ///
    pub server_id: u32,
    pub username: String,
    pub client_seed: u32,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub addon_info: Vec<AddonInfo>,
}

impl crate::Message for CMSG_AUTH_SESSION {
    const OPCODE: u32 = 0x01ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // build: u32
        w.write_all(&self.build.to_le_bytes())?;

        // server_id: u32
        w.write_all(&self.server_id.to_le_bytes())?;

        // username: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.username.as_bytes().iter().rev().next(), Some(&0_u8), "String `username` must not be null-terminated.");
        w.write_all(self.username.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // client_seed: u32
        w.write_all(&self.client_seed.to_le_bytes())?;

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_addon_info_size: u32
        w.write_all(&self.decompressed_addon_info_size.to_le_bytes())?;

        // addon_info: AddonInfo[-]
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.addon_info.iter() {
            let mut vec = Vec::new();
            i.write_into_vec(&mut vec)?;
            encoder.write_all(vec.as_slice());
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // build: u32
        let build = crate::util::read_u32_le(r)?;

        // server_id: u32
        let server_id = crate::util::read_u32_le(r)?;

        // username: CString
        let username = crate::util::read_c_string_to_vec(r)?;
        let username = String::from_utf8(username)?;

        // client_seed: u32
        let client_seed = crate::util::read_u32_le(r)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // decompressed_addon_info_size: u32
        let decompressed_addon_info_size = crate::util::read_u32_le(r)?;

        // addon_info: AddonInfo[-]
        let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

        let mut current_size = {
            4 // build: u32
            + 4 // server_id: u32
            + username.len() + 1 // username: CString
            + 4 // client_seed: u32
            + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
            + 4 // decompressed_addon_info_size: u32
        };
        let mut addon_info = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            let o = AddonInfo::read(decoder)?;
            current_size += o.size();
            addon_info.push(o);
        }

        Ok(Self {
            build,
            server_id,
            username,
            client_seed,
            client_proof,
            decompressed_addon_info_size,
            addon_info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AUTH_SESSION {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_AUTH_SESSION {}

impl CMSG_AUTH_SESSION {
    pub(crate) fn size(&self) -> usize {
        4 // build: u32
        + 4 // server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // client_seed: u32
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + self.addon_info.iter().fold(0, |acc, x| acc + x.size()) // addon_info: AddonInfo[-]
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test1 {
    use super::CMSG_AUTH_SESSION;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 190] = [ 0x00, 0xAC, 0xED, 0x01, 0x00, 0x00, 0xF3, 0x16, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x88, 0x02, 0xD8, 0x49, 0x88,
         0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7, 0x8A, 0xDB, 0xA4, 0xFB,
         0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, 0x56, 0x01, 0x00, 0x00, 0x78,
         0x9C, 0x6D, 0xCC, 0x4D, 0x0A, 0xC2, 0x40, 0x0C, 0x05, 0xE0, 0xBA, 0xF0,
         0x14, 0x7A, 0x19, 0xA7, 0x0B, 0x29, 0x38, 0x1B, 0x3B, 0xAE, 0x25, 0xCE,
         0xA4, 0x12, 0x9C, 0xA6, 0x25, 0xA6, 0xFE, 0xF4, 0x3E, 0x9E, 0xCB, 0xAB,
         0x28, 0x48, 0x51, 0x21, 0x6F, 0xFB, 0xDE, 0xF7, 0x5C, 0xA6, 0x71, 0x04,
         0x49, 0xFB, 0xD5, 0x10, 0x95, 0x3A, 0xDE, 0x55, 0xC5, 0xAC, 0xBD, 0x2E,
         0x37, 0xC5, 0x3B, 0x6E, 0xEA, 0x1C, 0xA8, 0x66, 0x6C, 0x08, 0x73, 0xF2,
         0xC4, 0xD4, 0x42, 0xFF, 0x19, 0x2D, 0x9E, 0xF3, 0xC7, 0x77, 0x44, 0x9C,
         0x88, 0x8F, 0xE6, 0x41, 0xD9, 0xB5, 0x07, 0xD0, 0x80, 0x37, 0x35, 0x60,
         0x29, 0xD0, 0xA8, 0xC9, 0xD6, 0xBE, 0x1E, 0xE4, 0x82, 0xF7, 0xA9, 0xFC,
         0x63, 0x15, 0x9F, 0x7B, 0x8C, 0x36, 0xF4, 0x10, 0xA5, 0x33, 0xD5, 0x16,
         0x28, 0x99, 0x24, 0x40, 0x46, 0x56, 0xD3, 0x04, 0x81, 0x84, 0xF5, 0x89,
         0x72, 0xB6, 0xA5, 0x00, 0x31, 0xCA, 0x2F, 0x7D, 0x01, 0xFB, 0xB4, 0x70,
         0x67, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test2 {
    use super::CMSG_AUTH_SESSION;
    use super::*;
    use super::super::*;
    use crate::world::tbc::opcodes::ClientOpcodeMessage;
    use crate::world::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 190] = [ 0x00, 0xAC, 0xED, 0x01, 0x00, 0x00, 0xF3, 0x16, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x88, 0x02, 0xD8, 0x49, 0x88,
         0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7, 0x8A, 0xDB, 0xA4, 0xFB,
         0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, 0x56, 0x01, 0x00, 0x00, 0x78,
         0x9C, 0x6D, 0xCC, 0x4D, 0x0A, 0xC2, 0x40, 0x0C, 0x05, 0xE0, 0xBA, 0xF0,
         0x14, 0x7A, 0x19, 0xA7, 0x0B, 0x29, 0x38, 0x1B, 0x3B, 0xAE, 0x25, 0xCE,
         0xA4, 0x12, 0x9C, 0xA6, 0x25, 0xA6, 0xFE, 0xF4, 0x3E, 0x9E, 0xCB, 0xAB,
         0x28, 0x48, 0x51, 0x21, 0x6F, 0xFB, 0xDE, 0xF7, 0x5C, 0xA6, 0x71, 0x04,
         0x49, 0xFB, 0xD5, 0x10, 0x95, 0x3A, 0xDE, 0x55, 0xC5, 0xAC, 0xBD, 0x2E,
         0x37, 0xC5, 0x3B, 0x6E, 0xEA, 0x1C, 0xA8, 0x66, 0x6C, 0x08, 0x73, 0xF2,
         0xC4, 0xD4, 0x42, 0xFF, 0x19, 0x2D, 0x9E, 0xF3, 0xC7, 0x77, 0x44, 0x9C,
         0x88, 0x8F, 0xE6, 0x41, 0xD9, 0xB5, 0x07, 0xD0, 0x80, 0x37, 0x35, 0x60,
         0x29, 0xD0, 0xA8, 0xC9, 0xD6, 0xBE, 0x1E, 0xE4, 0x82, 0xF7, 0xA9, 0xFC,
         0x63, 0x15, 0x9F, 0x7B, 0x8C, 0x36, 0xF4, 0x10, 0xA5, 0x33, 0xD5, 0x16,
         0x28, 0x99, 0x24, 0x40, 0x46, 0x56, 0xD3, 0x04, 0x81, 0x84, 0xF5, 0x89,
         0x72, 0xB6, 0xA5, 0x00, 0x31, 0xCA, 0x2F, 0x7D, 0x01, 0xFB, 0xB4, 0x70,
         0x67, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_AUTH_SESSION0() {
        let expected = CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0xA505DF1B,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

