use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_invite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_invite.wowm#L3):
/// ```text
/// smsg SMSG_GROUP_INVITE = 0x6F {
///     CString name;
/// }
/// ```
pub struct SMSG_GROUP_INVITE {
    pub name: String,
}

impl WorldServerMessageWrite for SMSG_GROUP_INVITE {
    const OPCODE: u16 = 0x6f;

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
impl WorldMessageBody for SMSG_GROUP_INVITE {
    type Error = SMSG_GROUP_INVITEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_GROUP_INVITE {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_GROUP_INVITE {
    fn maximum_possible_size() -> usize {
        256 // name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_GROUP_INVITEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GROUP_INVITEError {}
impl std::fmt::Display for SMSG_GROUP_INVITEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GROUP_INVITEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GROUP_INVITEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}
