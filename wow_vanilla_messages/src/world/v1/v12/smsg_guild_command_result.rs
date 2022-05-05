use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildCommand, GuildCommandError};
use crate::world::v1::v12::{GuildCommandResult, GuildCommandResultError};
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
pub struct SMSG_GUILD_COMMAND_RESULT {
    pub command: GuildCommand,
    pub string: String,
    pub result: GuildCommandResult,
}

impl ServerMessageWrite for SMSG_GUILD_COMMAND_RESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GUILD_COMMAND_RESULT {
    const OPCODE: u16 = 0x0093;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GUILD_COMMAND_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // command: GuildCommand
        let command = GuildCommand::read_u32_le(r)?;

        // string: CString
        let string = crate::util::read_c_string_to_vec(r)?;
        let string = String::from_utf8(string)?;

        // result: GuildCommandResult
        let result = GuildCommandResult::read_u32_le(r)?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // command: GuildCommand
        self.command.write_u32_le(w)?;

        // string: CString
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        self.result.write_u32_le(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // command: GuildCommand
        let command = GuildCommand::tokio_read_u32_le(r).await?;

        // string: CString
        let string = crate::util::tokio_read_c_string_to_vec(r).await?;
        let string = String::from_utf8(string)?;

        // result: GuildCommandResult
        let result = GuildCommandResult::tokio_read_u32_le(r).await?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // command: GuildCommand
        self.command.tokio_write_u32_le(w).await?;

        // string: CString
        w.write_all(self.string.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // result: GuildCommandResult
        self.result.tokio_write_u32_le(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // command: GuildCommand
        let command = GuildCommand::astd_read_u32_le(r).await?;

        // string: CString
        let string = crate::util::astd_read_c_string_to_vec(r).await?;
        let string = String::from_utf8(string)?;

        // result: GuildCommandResult
        let result = GuildCommandResult::astd_read_u32_le(r).await?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // command: GuildCommand
        self.command.astd_write_u32_le(w).await?;

        // string: CString
        w.write_all(self.string.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // result: GuildCommandResult
        self.result.astd_write_u32_le(w).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_GUILD_COMMAND_RESULT {
    fn size(&self) -> usize {
        4 // command: GuildCommand upcasted to u32
        + self.string.len() + 1 // string: CString and Null Terminator
        + 4 // result: GuildCommandResult upcasted to u32
    }
}

impl MaximumPossibleSized for SMSG_GUILD_COMMAND_RESULT {
    fn maximum_possible_size() -> usize {
        GuildCommand::maximum_possible_size() // command: GuildCommand
        + 256 // string: CString
        + GuildCommandResult::maximum_possible_size() // result: GuildCommandResult
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_COMMAND_RESULTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GuildCommand(GuildCommandError),
    GuildCommandResult(GuildCommandResultError),
}

impl std::error::Error for SMSG_GUILD_COMMAND_RESULTError {}
impl std::fmt::Display for SMSG_GUILD_COMMAND_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GuildCommand(i) => i.fmt(f),
            Self::GuildCommandResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GuildCommandError> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: GuildCommandError) -> Self {
        Self::GuildCommand(e)
    }
}

impl From<GuildCommandResultError> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: GuildCommandResultError) -> Self {
        Self::GuildCommandResult(e)
    }
}

