use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_PET_RENAME {
    pub pet_guid: Guid,
    pub name: String,
}

impl ClientMessageWrite for CMSG_PET_RENAME {}

impl MessageBody for CMSG_PET_RENAME {
    const OPCODE: u16 = 0x0177;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_PET_RENAMEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            pet_guid,
            name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.write(w)?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_PET_RENAME {
    fn size(&self) -> usize {
        8 // pet_guid: Guid
        + self.name.len() + 1 // name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_PET_RENAME {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: Guid
        + 256 // name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_PET_RENAMEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_PET_RENAMEError {}
impl std::fmt::Display for CMSG_PET_RENAMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_PET_RENAMEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_PET_RENAMEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

