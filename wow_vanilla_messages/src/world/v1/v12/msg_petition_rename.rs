use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_PETITION_RENAME {
    pub petition_guid: Guid,
    pub new_name: String,
}

impl ClientMessageWrite for MSG_PETITION_RENAME {}

impl ServerMessageWrite for MSG_PETITION_RENAME {}

impl MessageBody for MSG_PETITION_RENAME {
    const OPCODE: u16 = 0x02c1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = MSG_PETITION_RENAMEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // new_name: CString
        let new_name = crate::util::read_c_string_to_vec(r)?;
        let new_name = String::from_utf8(new_name)?;

        Ok(Self {
            petition_guid,
            new_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // new_name: CString
        w.write_all(self.new_name.as_bytes())?;
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
            // petition_guid: Guid
            let petition_guid = Guid::tokio_read(r).await?;

            // new_name: CString
            let new_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let new_name = String::from_utf8(new_name)?;

            Ok(Self {
                petition_guid,
                new_name,
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
            // petition_guid: Guid
            self.petition_guid.tokio_write(w).await?;

            // new_name: CString
            w.write_all(self.new_name.as_bytes()).await?;
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
            // petition_guid: Guid
            let petition_guid = Guid::astd_read(r).await?;

            // new_name: CString
            let new_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let new_name = String::from_utf8(new_name)?;

            Ok(Self {
                petition_guid,
                new_name,
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
            // petition_guid: Guid
            self.petition_guid.astd_write(w).await?;

            // new_name: CString
            w.write_all(self.new_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for MSG_PETITION_RENAME {
    fn size(&self) -> usize {
        0
        + 8 // petition_guid: Guid
        + self.new_name.len() + 1 // new_name: CString
    }
}

impl MaximumPossibleSized for MSG_PETITION_RENAME {
    fn maximum_possible_size() -> usize {
        0
        + 8 // petition_guid: Guid
        + 256 // new_name: CString
    }
}

#[derive(Debug)]
pub enum MSG_PETITION_RENAMEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for MSG_PETITION_RENAMEError {}
impl std::fmt::Display for MSG_PETITION_RENAMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_PETITION_RENAMEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for MSG_PETITION_RENAMEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

