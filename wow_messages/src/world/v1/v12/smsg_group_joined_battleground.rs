use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BgTypeId, BgTypeIdError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_group_joined_battleground.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_group_joined_battleground.wowm#L21):
/// ```text
/// smsg SMSG_GROUP_JOINED_BATTLEGROUND = 0x2E8 {
///     BgTypeId id;
/// }
/// ```
pub struct SMSG_GROUP_JOINED_BATTLEGROUND {
    pub id: BgTypeId,
}

impl WorldServerMessageWrite for SMSG_GROUP_JOINED_BATTLEGROUND {
    const OPCODE: u16 = 0x2e8;

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
impl WorldMessageBody for SMSG_GROUP_JOINED_BATTLEGROUND {
    type Error = SMSG_GROUP_JOINED_BATTLEGROUNDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: BgTypeId
        let id = BgTypeId::read(r)?;

        Ok(Self {
            id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: BgTypeId
        self.id.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn maximum_possible_size() -> usize {
        BgTypeId::size() // id: BgTypeId
    }
}

#[derive(Debug)]
pub enum SMSG_GROUP_JOINED_BATTLEGROUNDError {
    Io(std::io::Error),
    BgTypeId(BgTypeIdError),
}

impl std::error::Error for SMSG_GROUP_JOINED_BATTLEGROUNDError {}
impl std::fmt::Display for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BgTypeId(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BgTypeIdError> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e: BgTypeIdError) -> Self {
        Self::BgTypeId(e)
    }
}

