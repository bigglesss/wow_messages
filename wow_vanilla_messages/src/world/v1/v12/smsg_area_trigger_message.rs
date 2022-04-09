use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm#L3):
/// ```text
/// smsg SMSG_AREA_TRIGGER_MESSAGE = 0x2B8 {
///     u32 length;
///     CString message;
/// }
/// ```
pub struct SMSG_AREA_TRIGGER_MESSAGE {
    pub length: u32,
    pub message: String,
}

impl WorldServerMessageWrite for SMSG_AREA_TRIGGER_MESSAGE {
    const OPCODE: u16 = 0x2b8;

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
impl WorldMessageBody for SMSG_AREA_TRIGGER_MESSAGE {
    type Error = SMSG_AREA_TRIGGER_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // length: u32
        let length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            length,
            message,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // length: u32
        w.write_all(&self.length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_AREA_TRIGGER_MESSAGE {
    fn size(&self) -> usize {
        4 // length: u32
        + self.message.len() + 1 // message: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_AREA_TRIGGER_MESSAGE {
    fn maximum_possible_size() -> usize {
        4 // length: u32
        + 256 // message: CString
    }
}

#[derive(Debug)]
pub enum SMSG_AREA_TRIGGER_MESSAGEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_AREA_TRIGGER_MESSAGEError {}
impl std::fmt::Display for SMSG_AREA_TRIGGER_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_AREA_TRIGGER_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_AREA_TRIGGER_MESSAGEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}
