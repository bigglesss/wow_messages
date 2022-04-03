use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new2.wowm:653`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new2.wowm#L653):
/// ```text
/// enum GmTicketResponse : u32 {
///     NOT_EXIST = 0;
///     ALREADY_EXIST = 1;
///     CREATE_SUCCESS = 2;
///     CREATE_ERROR = 3;
///     UPDATE_SUCCESS = 4;
///     UPDATE_ERROR = 5;
///     TICKET_DELETED = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketResponse {
    NOT_EXIST,
    ALREADY_EXIST,
    CREATE_SUCCESS,
    CREATE_ERROR,
    UPDATE_SUCCESS,
    UPDATE_ERROR,
    TICKET_DELETED,
}

impl ReadableAndWritable for GmTicketResponse {
    type Error = GmTicketResponseError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl GmTicketResponse {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketResponseError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketResponseError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketResponseError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::NOT_EXIST => 0x0,
            Self::ALREADY_EXIST => 0x1,
            Self::CREATE_SUCCESS => 0x2,
            Self::CREATE_ERROR => 0x3,
            Self::UPDATE_SUCCESS => 0x4,
            Self::UPDATE_ERROR => 0x5,
            Self::TICKET_DELETED => 0x9,
        }
    }

    pub const fn new() -> Self {
        Self::NOT_EXIST
    }

}

impl ConstantSized for GmTicketResponse {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmTicketResponse {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for GmTicketResponse {
    fn default() -> Self {
        Self::NOT_EXIST
    }
}

impl std::fmt::Display for GmTicketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_EXIST => f.write_str("NOT_EXIST"),
            Self::ALREADY_EXIST => f.write_str("ALREADY_EXIST"),
            Self::CREATE_SUCCESS => f.write_str("CREATE_SUCCESS"),
            Self::CREATE_ERROR => f.write_str("CREATE_ERROR"),
            Self::UPDATE_SUCCESS => f.write_str("UPDATE_SUCCESS"),
            Self::UPDATE_ERROR => f.write_str("UPDATE_ERROR"),
            Self::TICKET_DELETED => f.write_str("TICKET_DELETED"),
        }
    }
}

impl TryFrom<u32> for GmTicketResponse {
    type Error = TryFromGmTicketResponseError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_EXIST),
            1 => Ok(Self::ALREADY_EXIST),
            2 => Ok(Self::CREATE_SUCCESS),
            3 => Ok(Self::CREATE_ERROR),
            4 => Ok(Self::UPDATE_SUCCESS),
            5 => Ok(Self::UPDATE_ERROR),
            9 => Ok(Self::TICKET_DELETED),
            _ => Err(TryFromGmTicketResponseError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGmTicketResponseError {
    value: u32,
}

impl TryFromGmTicketResponseError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GmTicketResponseError {
    Read(std::io::Error),
    TryFrom(TryFromGmTicketResponseError),
}

impl std::error::Error for GmTicketResponseError {}
impl std::fmt::Display for TryFromGmTicketResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketResponse': '{}'", self.value))
    }
}

impl std::fmt::Display for GmTicketResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GmTicketResponseError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGmTicketResponseError> for GmTicketResponseError {
    fn from(value: TryFromGmTicketResponseError) -> Self {
        Self::TryFrom(value)
    }
}

