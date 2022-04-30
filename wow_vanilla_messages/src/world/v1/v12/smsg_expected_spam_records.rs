use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_EXPECTED_SPAM_RECORDS {
    pub records: Vec<String>,
}

impl ServerMessageWrite for SMSG_EXPECTED_SPAM_RECORDS {}

impl MessageBody for SMSG_EXPECTED_SPAM_RECORDS {
    const OPCODE: u16 = 0x0332;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_EXPECTED_SPAM_RECORDSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_records: u32
        let amount_of_records = crate::util::read_u32_le(r)?;

        // records: CString[amount_of_records]
        let mut records = Vec::with_capacity(amount_of_records as usize);
        for i in 0..amount_of_records {
            let s = crate::util::read_c_string_to_vec(r)?;
            records.push(String::from_utf8(s)?);
        }

        Ok(Self {
            records,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_EXPECTED_SPAM_RECORDS {
    fn size(&self) -> usize {
        4 // amount_of_records: u32
        + self.records.iter().fold(0, |acc, x| acc + x.len() + 1) // records: CString[amount_of_records]
    }
}

impl MaximumPossibleSized for SMSG_EXPECTED_SPAM_RECORDS {
    fn maximum_possible_size() -> usize {
        4 // amount_of_records: u32
        + 4294967295 * 256 // records: CString[amount_of_records]
    }
}

#[derive(Debug)]
pub enum SMSG_EXPECTED_SPAM_RECORDSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_EXPECTED_SPAM_RECORDSError {}
impl std::fmt::Display for SMSG_EXPECTED_SPAM_RECORDSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_EXPECTED_SPAM_RECORDSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_EXPECTED_SPAM_RECORDSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

