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
pub struct CMSG_GROUP_SWAP_SUB_GROUP {
    pub name: String,
    pub swap_with_name: String,
}

impl ClientMessageWrite for CMSG_GROUP_SWAP_SUB_GROUP {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_GROUP_SWAP_SUB_GROUP {
    const OPCODE: u16 = 0x0280;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GROUP_SWAP_SUB_GROUPError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // swap_with_name: CString
        let swap_with_name = crate::util::read_c_string_to_vec(r)?;
        let swap_with_name = String::from_utf8(swap_with_name)?;

        Ok(Self {
            name,
            swap_with_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // swap_with_name: CString
        w.write_all(self.swap_with_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // swap_with_name: CString
        let swap_with_name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let swap_with_name = String::from_utf8(swap_with_name)?;

        Ok(Self {
            name,
            swap_with_name,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // swap_with_name: CString
        w.write_all(self.swap_with_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // swap_with_name: CString
        let swap_with_name = crate::util::astd_read_c_string_to_vec(r).await?;
        let swap_with_name = String::from_utf8(swap_with_name)?;

        Ok(Self {
            name,
            swap_with_name,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // swap_with_name: CString
        w.write_all(self.swap_with_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }
}

impl VariableSized for CMSG_GROUP_SWAP_SUB_GROUP {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + self.swap_with_name.len() + 1 // swap_with_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GROUP_SWAP_SUB_GROUP {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + 256 // swap_with_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GROUP_SWAP_SUB_GROUPError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GROUP_SWAP_SUB_GROUPError {}
impl std::fmt::Display for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

