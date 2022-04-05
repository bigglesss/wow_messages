use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SheathState, SheathStateError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_setsheathed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_setsheathed.wowm#L3):
/// ```text
/// cmsg CMSG_SETSHEATHED = 0x1E0 {
///     SheathState sheathed;
/// }
/// ```
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl WorldClientMessageWrite for CMSG_SETSHEATHED {
    const OPCODE: u32 = 0x1e0;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_SETSHEATHED {
    type Error = CMSG_SETSHEATHEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sheathed: SheathState
        let sheathed = SheathState::read_u32_le(r)?;

        Ok(Self {
            sheathed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sheathed: SheathState
        self.sheathed.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SETSHEATHED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SETSHEATHED {
    fn maximum_possible_size() -> usize {
        4 // sheathed: SheathState upcasted to u32
    }
}

#[derive(Debug)]
pub enum CMSG_SETSHEATHEDError {
    Io(std::io::Error),
    SheathState(SheathStateError),
}

impl std::error::Error for CMSG_SETSHEATHEDError {}
impl std::fmt::Display for CMSG_SETSHEATHEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SheathState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SETSHEATHEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SheathStateError> for CMSG_SETSHEATHEDError {
    fn from(e: SheathStateError) -> Self {
        Self::SheathState(e)
    }
}

