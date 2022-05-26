use crate::{ServerMessage, ClientMessage};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::logon::version_3::CMD_AUTH_LOGON_CHALLENGE_Server;
use crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Server;
use crate::logon::version_2::CMD_REALM_LIST_Server;
use crate::logon::version_3::CMD_XFER_INITIATE;
use crate::logon::version_3::CMD_XFER_DATA;

#[derive(Debug)]
pub enum ServerOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server),
    CMD_REALM_LIST(CMD_REALM_LIST_Server),
    CMD_XFER_INITIATE(CMD_XFER_INITIATE),
    CMD_XFER_DATA(CMD_XFER_DATA),
}

impl ServerOpcodeMessage {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_REALM_LIST(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_XFER_INITIATE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_XFER_DATA(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
        }

        Ok(w)
    }
}

impl ServerOpcodeMessage {
    #[cfg(feature = "sync")]
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::read(r)?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::read(r)?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::read(r)?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::tokio_read(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::tokio_read(r).await?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::tokio_read(r).await?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ServerOpcodeMessageError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Server::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Server::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Server::astd_read(r).await?)),
            0x30 => Ok(Self::CMD_XFER_INITIATE(CMD_XFER_INITIATE::astd_read(r).await?)),
            0x31 => Ok(Self::CMD_XFER_DATA(CMD_XFER_DATA::astd_read(r).await?)),
            opcode => Err(ServerOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ServerOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for ServerOpcodeMessageError {}
impl std::fmt::Display for ServerOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ServerMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ServerOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::ParseError> for ServerOpcodeMessageError {
    fn from(e: crate::errors::ParseError) -> Self {
        match e {
            crate::errors::ParseError::Io(i) => Self::Io(i),
            crate::errors::ParseError::Enum(i) => Self::Enum(i),
            crate::errors::ParseError::String(i) => Self::String(i),
        }
    }
}

use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::version_3::CMD_AUTH_LOGON_PROOF_Client;
use crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
use crate::logon::version_3::CMD_SURVEY_RESULT;
use crate::logon::version_2::CMD_REALM_LIST_Client;
use crate::logon::version_3::CMD_XFER_ACCEPT;
use crate::logon::version_3::CMD_XFER_RESUME;
use crate::logon::version_3::CMD_XFER_CANCEL;

#[derive(Debug)]
pub enum ClientOpcodeMessage {
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client),
    CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client),
    CMD_SURVEY_RESULT(CMD_SURVEY_RESULT),
    CMD_REALM_LIST(CMD_REALM_LIST_Client),
    CMD_XFER_ACCEPT(CMD_XFER_ACCEPT),
    CMD_XFER_RESUME(CMD_XFER_RESUME),
    CMD_XFER_CANCEL(CMD_XFER_CANCEL),
}

impl ClientOpcodeMessage {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        match self {
            Self::CMD_AUTH_LOGON_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_LOGON_PROOF(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_AUTH_RECONNECT_CHALLENGE(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_SURVEY_RESULT(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_REALM_LIST(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_XFER_ACCEPT(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_XFER_RESUME(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
            Self::CMD_XFER_CANCEL(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,
        }

        Ok(w)
    }
}

impl ClientOpcodeMessage {
    #[cfg(feature = "sync")]
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::read_u8_le(r)?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::read(r)?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?)),
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::read(r)?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::read(r)?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::read(r)?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::read(r)?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::read(r)?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::tokio_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::tokio_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?)),
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::tokio_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::tokio_read(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::tokio_read(r).await?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::tokio_read(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::tokio_read(r).await?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ClientOpcodeMessageError> {
        let opcode = crate::util::astd_read_u8_le(r).await?;
        match opcode {
            0x00 => Ok(Self::CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_Client::astd_read(r).await?)),
            0x01 => Ok(Self::CMD_AUTH_LOGON_PROOF(CMD_AUTH_LOGON_PROOF_Client::astd_read(r).await?)),
            0x02 => Ok(Self::CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read(r).await?)),
            0x04 => Ok(Self::CMD_SURVEY_RESULT(CMD_SURVEY_RESULT::astd_read(r).await?)),
            0x10 => Ok(Self::CMD_REALM_LIST(CMD_REALM_LIST_Client::astd_read(r).await?)),
            0x32 => Ok(Self::CMD_XFER_ACCEPT(CMD_XFER_ACCEPT::astd_read(r).await?)),
            0x33 => Ok(Self::CMD_XFER_RESUME(CMD_XFER_RESUME::astd_read(r).await?)),
            0x34 => Ok(Self::CMD_XFER_CANCEL(CMD_XFER_CANCEL::astd_read(r).await?)),
            opcode => Err(ClientOpcodeMessageError::InvalidOpcode(opcode)),
        }
    }

}

#[derive(Debug)]
pub enum ClientOpcodeMessageError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for ClientOpcodeMessageError {}
impl std::fmt::Display for ClientOpcodeMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for ClientMessage: '{}'", i)),
        }
    }
}

impl From<std::io::Error> for ClientOpcodeMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::ParseError> for ClientOpcodeMessageError {
    fn from(e: crate::errors::ParseError) -> Self {
        match e {
            crate::errors::ParseError::Io(i) => Self::Io(i),
            crate::errors::ParseError::Enum(i) => Self::Enum(i),
            crate::errors::ParseError::String(i) => Self::String(i),
        }
    }
}

