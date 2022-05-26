use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub item_text_id: u32,
    pub text: String,
}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0244;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_ITEM_TEXT_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        Ok(Self {
            item_text_id,
            text,
        })
    }

    #[cfg(feature = "tokio")]
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
            // item_text_id: u32
            let item_text_id = crate::util::tokio_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            Ok(Self {
                item_text_id,
                text,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // item_text_id: u32
            let item_text_id = crate::util::astd_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::astd_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            Ok(Self {
                item_text_id,
                text,
            })
        })
    }

}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 4 // item_text_id: u32
        + self.text.len() + 1 // text: CString
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_ITEM_TEXT_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_TEXT_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

