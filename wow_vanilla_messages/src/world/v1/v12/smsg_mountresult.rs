use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MountResult, MountResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_mountresult.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_mountresult.wowm#L3):
/// ```text
/// smsg SMSG_MOUNTRESULT = 0x16E {
///     MountResult result;
/// }
/// ```
pub struct SMSG_MOUNTRESULT {
    pub result: MountResult,
}

impl WorldServerMessageWrite for SMSG_MOUNTRESULT {
    const OPCODE: u16 = 0x16e;

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
impl WorldMessageBody for SMSG_MOUNTRESULT {
    type Error = SMSG_MOUNTRESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: MountResult
        let result = MountResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: MountResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_MOUNTRESULT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_MOUNTRESULT {
    fn maximum_possible_size() -> usize {
        MountResult::size() // result: MountResult
    }
}

#[derive(Debug)]
pub enum SMSG_MOUNTRESULTError {
    Io(std::io::Error),
    MountResult(MountResultError),
}

impl std::error::Error for SMSG_MOUNTRESULTError {}
impl std::fmt::Display for SMSG_MOUNTRESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MountResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MOUNTRESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MountResultError> for SMSG_MOUNTRESULTError {
    fn from(e: MountResultError) -> Self {
        Self::MountResult(e)
    }
}
