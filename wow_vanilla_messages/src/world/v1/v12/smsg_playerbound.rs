use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Area, AreaError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PLAYERBOUND {
    pub guid: Guid,
    pub area: Area,
}

impl WorldServerMessageWrite for SMSG_PLAYERBOUND {
    const OPCODE: u16 = 0x158;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PLAYERBOUND {
    type Error = SMSG_PLAYERBOUNDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // area: Area
        let area = Area::read(r)?;

        Ok(Self {
            guid,
            area,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // area: Area
        self.area.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PLAYERBOUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PLAYERBOUND {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + Area::size() // area: Area
    }
}

#[derive(Debug)]
pub enum SMSG_PLAYERBOUNDError {
    Io(std::io::Error),
    Area(AreaError),
}

impl std::error::Error for SMSG_PLAYERBOUNDError {}
impl std::fmt::Display for SMSG_PLAYERBOUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PLAYERBOUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_PLAYERBOUNDError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

