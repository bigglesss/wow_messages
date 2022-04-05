use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MeetingStoneFailure, MeetingStoneFailureError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:3133`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L3133):
/// ```text
/// smsg SMSG_MEETINGSTONE_JOINFAILED = 0x2BB {
///     MeetingStoneFailure reason;
/// }
/// ```
pub struct SMSG_MEETINGSTONE_JOINFAILED {
    pub reason: MeetingStoneFailure,
}

impl WorldServerMessageWrite for SMSG_MEETINGSTONE_JOINFAILED {
    const OPCODE: u16 = 0x2bb;

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
impl WorldMessageBody for SMSG_MEETINGSTONE_JOINFAILED {
    type Error = SMSG_MEETINGSTONE_JOINFAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: MeetingStoneFailure
        let reason = MeetingStoneFailure::read(r)?;

        Ok(Self {
            reason,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        self.reason.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_MEETINGSTONE_JOINFAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_JOINFAILED {
    fn maximum_possible_size() -> usize {
        MeetingStoneFailure::size() // reason: MeetingStoneFailure
    }
}

#[derive(Debug)]
pub enum SMSG_MEETINGSTONE_JOINFAILEDError {
    Io(std::io::Error),
    MeetingStoneFailure(MeetingStoneFailureError),
}

impl std::error::Error for SMSG_MEETINGSTONE_JOINFAILEDError {}
impl std::fmt::Display for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MeetingStoneFailure(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MeetingStoneFailureError> for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn from(e: MeetingStoneFailureError) -> Self {
        Self::MeetingStoneFailure(e)
    }
}

