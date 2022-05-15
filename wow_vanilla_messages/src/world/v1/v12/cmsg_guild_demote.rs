use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_DEMOTE {
    pub player_name: String,
}

impl ClientMessageWrite for CMSG_GUILD_DEMOTE {}

impl MessageBody for CMSG_GUILD_DEMOTE {
    const OPCODE: u16 = 0x008c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GUILD_DEMOTEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        Ok(Self {
            player_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // player_name: CString
            let player_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            Ok(Self {
                player_name,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // player_name: CString
            w.write_all(self.player_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // player_name: CString
            let player_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            Ok(Self {
                player_name,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // player_name: CString
            w.write_all(self.player_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl CMSG_GUILD_DEMOTE {
    pub fn size(&self) -> usize {
        0
        + self.player_name.len() + 1 // player_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_DEMOTEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_DEMOTEError {}
impl std::fmt::Display for CMSG_GUILD_DEMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_DEMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_DEMOTEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

