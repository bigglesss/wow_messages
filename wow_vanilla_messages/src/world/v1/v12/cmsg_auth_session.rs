use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L3):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x1ED {
///     u32 build;
///     u32 server_id;
///     CString username;
///     u32 client_seed;
///     u8[20] client_proof;
///     u32 decompressed_addon_info_size;
///     u8[-] compressed_addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub build: u32,
    pub server_id: u32,
    pub username: String,
    pub client_seed: u32,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub compressed_addon_info: Vec<u8>,
}

impl WorldClientMessageWrite for CMSG_AUTH_SESSION {
    const OPCODE: u32 = 0x1ed;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_AUTH_SESSION {
    type Error = CMSG_AUTH_SESSIONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

        // compressed_addon_info: u8[-]
        let mut current_size = {
            4 // build: u32
            + 4 // server_id: u32
            + username.len() + 1 // username: CString and Null Terminator
            + 4 // client_seed: u32
            + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
            + 4 // decompressed_addon_info_size: u32
        };
        let mut compressed_addon_info = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            compressed_addon_info.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            build,
            server_id,
            username,
            client_seed,
            client_proof,
            decompressed_addon_info_size,
            compressed_addon_info,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // build: u32
        w.write_all(&self.build.to_le_bytes())?;

        // server_id: u32
        w.write_all(&self.server_id.to_le_bytes())?;

        // username: CString
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

        // compressed_addon_info: u8[-]
        for i in self.compressed_addon_info.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl VariableSized for CMSG_AUTH_SESSION {
    fn size(&self) -> usize {
        4 // build: u32
        + 4 // server_id: u32
        + self.username.len() + 1 // username: CString and Null Terminator
        + 4 // client_seed: u32
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + self.compressed_addon_info.len() * core::mem::size_of::<u8>() // compressed_addon_info: u8[-]
    }
}

impl MaximumPossibleSized for CMSG_AUTH_SESSION {
    fn maximum_possible_size() -> usize {
        4 // build: u32
        + 4 // server_id: u32
        + 256 // username: CString
        + 4 // client_seed: u32
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + 65536 // compressed_addon_info: u8[-]
    }
}

#[derive(Debug)]
pub enum CMSG_AUTH_SESSIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_AUTH_SESSIONError {}
impl std::fmt::Display for CMSG_AUTH_SESSIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_AUTH_SESSIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_AUTH_SESSIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}
