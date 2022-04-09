use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_rename.wowm#L3):
/// ```text
/// msg MSG_PETITION_RENAME = 0x2C1 {
///     Guid petition_guid;
///     CString new_name;
/// }
/// ```
pub struct MSG_PETITION_RENAME {
    pub petition_guid: Guid,
    pub new_name: String,
}

impl WorldClientMessageWrite for MSG_PETITION_RENAME {
    const OPCODE: u32 = 0x2c1;

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
impl WorldServerMessageWrite for MSG_PETITION_RENAME {
    const OPCODE: u16 = 0x2c1;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_PETITION_RENAME {
    type Error = MSG_PETITION_RENAMEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // new_name: CString
        let new_name = crate::util::read_c_string_to_vec(r)?;
        let new_name = String::from_utf8(new_name)?;

        Ok(Self {
            petition_guid,
            new_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // new_name: CString
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for MSG_PETITION_RENAME {
    fn size(&self) -> usize {
        8 // petition_guid: Guid
        + self.new_name.len() + 1 // new_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for MSG_PETITION_RENAME {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: Guid
        + 256 // new_name: CString
    }
}

#[derive(Debug)]
pub enum MSG_PETITION_RENAMEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for MSG_PETITION_RENAMEError {}
impl std::fmt::Display for MSG_PETITION_RENAMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_PETITION_RENAMEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for MSG_PETITION_RENAMEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}
