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
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
}

impl ClientMessageWrite for CMSG_GUILD_RANK {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_GUILD_RANK {
    const OPCODE: u16 = 0x0231;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GUILD_RANKError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // rank_id: u32
        let rank_id = crate::util::read_u32_le(r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // rank_name: CString
        let rank_name = crate::util::read_c_string_to_vec(r)?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // rank_id: u32
        let rank_id = crate::util::tokio_read_u32_le(r).await?;

        // rights: u32
        let rights = crate::util::tokio_read_u32_le(r).await?;

        // rank_name: CString
        let rank_name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes()).await?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes()).await?;

        // rank_name: CString
        w.write_all(self.rank_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // rank_id: u32
        let rank_id = crate::util::astd_read_u32_le(r).await?;

        // rights: u32
        let rights = crate::util::astd_read_u32_le(r).await?;

        // rank_name: CString
        let rank_name = crate::util::astd_read_c_string_to_vec(r).await?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes()).await?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes()).await?;

        // rank_name: CString
        w.write_all(self.rank_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_RANK {
    fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_RANK {
    fn maximum_possible_size() -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + 256 // rank_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_RANKError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_RANKError {}
impl std::fmt::Display for CMSG_GUILD_RANKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_RANKError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_RANKError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

