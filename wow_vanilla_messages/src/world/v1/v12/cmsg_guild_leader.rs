use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_LEADER {
    pub new_guild_leader_name: String,
}

impl ClientMessageWrite for CMSG_GUILD_LEADER {}

impl MessageBody for CMSG_GUILD_LEADER {
    const OPCODE: u16 = 0x0090;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GUILD_LEADERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_guild_leader_name: CString
        let new_guild_leader_name = crate::util::read_c_string_to_vec(r)?;
        let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

        Ok(Self {
            new_guild_leader_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // new_guild_leader_name: CString
        w.write_all(self.new_guild_leader_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // new_guild_leader_name: CString
            let new_guild_leader_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

            Ok(Self {
                new_guild_leader_name,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // new_guild_leader_name: CString
            w.write_all(self.new_guild_leader_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // new_guild_leader_name: CString
            let new_guild_leader_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

            Ok(Self {
                new_guild_leader_name,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // new_guild_leader_name: CString
            w.write_all(self.new_guild_leader_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for CMSG_GUILD_LEADER {
    fn size(&self) -> usize {
        0
        + self.new_guild_leader_name.len() + 1 // new_guild_leader_name: CString
    }
}

impl MaximumPossibleSized for CMSG_GUILD_LEADER {
    fn maximum_possible_size() -> usize {
        0
        + 256 // new_guild_leader_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_LEADERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_LEADERError {}
impl std::fmt::Display for CMSG_GUILD_LEADERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_LEADERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_LEADERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

