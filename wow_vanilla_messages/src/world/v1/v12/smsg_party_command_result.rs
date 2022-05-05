use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PartyOperation, PartyOperationError};
use crate::world::v1::v12::{PartyResult, PartyResultError};
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
pub struct SMSG_PARTY_COMMAND_RESULT {
    pub operation: PartyOperation,
    pub member: String,
    pub result: PartyResult,
}

impl ServerMessageWrite for SMSG_PARTY_COMMAND_RESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PARTY_COMMAND_RESULT {
    const OPCODE: u16 = 0x007f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_PARTY_COMMAND_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation = PartyOperation::read_u32_le(r)?;

        // member: CString
        let member = crate::util::read_c_string_to_vec(r)?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result = PartyResult::read_u32_le(r)?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: PartyOperation
        self.operation.write_u32_le(w)?;

        // member: CString
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        self.result.write_u32_le(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation = PartyOperation::tokio_read_u32_le(r).await?;

        // member: CString
        let member = crate::util::tokio_read_c_string_to_vec(r).await?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result = PartyResult::tokio_read_u32_le(r).await?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: PartyOperation
        self.operation.tokio_write_u32_le(w).await?;

        // member: CString
        w.write_all(self.member.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // result: PartyResult
        self.result.tokio_write_u32_le(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation = PartyOperation::astd_read_u32_le(r).await?;

        // member: CString
        let member = crate::util::astd_read_c_string_to_vec(r).await?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result = PartyResult::astd_read_u32_le(r).await?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: PartyOperation
        self.operation.astd_write_u32_le(w).await?;

        // member: CString
        w.write_all(self.member.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // result: PartyResult
        self.result.astd_write_u32_le(w).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_PARTY_COMMAND_RESULT {
    fn size(&self) -> usize {
        4 // operation: PartyOperation upcasted to u32
        + self.member.len() + 1 // member: CString and Null Terminator
        + 4 // result: PartyResult upcasted to u32
    }
}

impl MaximumPossibleSized for SMSG_PARTY_COMMAND_RESULT {
    fn maximum_possible_size() -> usize {
        PartyOperation::maximum_possible_size() // operation: PartyOperation
        + 256 // member: CString
        + PartyResult::maximum_possible_size() // result: PartyResult
    }
}

#[derive(Debug)]
pub enum SMSG_PARTY_COMMAND_RESULTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    PartyOperation(PartyOperationError),
    PartyResult(PartyResultError),
}

impl std::error::Error for SMSG_PARTY_COMMAND_RESULTError {}
impl std::fmt::Display for SMSG_PARTY_COMMAND_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::PartyOperation(i) => i.fmt(f),
            Self::PartyResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<PartyOperationError> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: PartyOperationError) -> Self {
        Self::PartyOperation(e)
    }
}

impl From<PartyResultError> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: PartyResultError) -> Self {
        Self::PartyResult(e)
    }
}

