use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3076`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3076):
/// ```text
/// smsg SMSG_RESURRECT_REQUEST = 0x15B {
///     u64 guid;
///     u32 name_length;
///     CString name;
///     u8 caster_is_spirit_healer;
///     u8 respect_resurrection_timer;
/// }
/// ```
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: u64,
    pub name_length: u32,
    pub name: String,
    pub caster_is_spirit_healer: u8,
    pub respect_resurrection_timer: u8,
}

impl WorldServerMessageWrite for SMSG_RESURRECT_REQUEST {
    const OPCODE: u16 = 0x15b;

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
impl WorldMessageBody for SMSG_RESURRECT_REQUEST {
    type Error = SMSG_RESURRECT_REQUESTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // name_length: u32
        let name_length = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // caster_is_spirit_healer: u8
        let caster_is_spirit_healer = crate::util::read_u8_le(r)?;

        // respect_resurrection_timer: u8
        let respect_resurrection_timer = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            name_length,
            name,
            caster_is_spirit_healer,
            respect_resurrection_timer,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // name_length: u32
        w.write_all(&self.name_length.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // caster_is_spirit_healer: u8
        w.write_all(&self.caster_is_spirit_healer.to_le_bytes())?;

        // respect_resurrection_timer: u8
        w.write_all(&self.respect_resurrection_timer.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_RESURRECT_REQUEST {
    fn size(&self) -> usize {
        8 // guid: u64
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString and Null Terminator
        + 1 // caster_is_spirit_healer: u8
        + 1 // respect_resurrection_timer: u8
    }
}

impl MaximumPossibleSized for SMSG_RESURRECT_REQUEST {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // name_length: u32
        + 256 // name: CString
        + 1 // caster_is_spirit_healer: u8
        + 1 // respect_resurrection_timer: u8
    }
}

#[derive(Debug)]
pub enum SMSG_RESURRECT_REQUESTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_RESURRECT_REQUESTError {}
impl std::fmt::Display for SMSG_RESURRECT_REQUESTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RESURRECT_REQUESTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_RESURRECT_REQUESTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

