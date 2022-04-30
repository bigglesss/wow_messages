use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketType, GmTicketTypeError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GMTICKET_CREATE {
    pub category: CMSG_GMTICKET_CREATEGmTicketType,
    pub map: Map,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub message: String,
    pub reserved_for_future_use: String,
}

impl ClientMessageWrite for CMSG_GMTICKET_CREATE {}

impl MessageBody for CMSG_GMTICKET_CREATE {
    const OPCODE: u16 = 0x0205;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GMTICKET_CREATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // category: GmTicketType
        let category = GmTicketType::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        // reserved_for_future_use: CString
        let reserved_for_future_use = crate::util::read_c_string_to_vec(r)?;
        let reserved_for_future_use = String::from_utf8(reserved_for_future_use)?;

        let category_if = match category {
            GmTicketType::STUCK => CMSG_GMTICKET_CREATEGmTicketType::STUCK,
            GmTicketType::BEHAVIOR_HARASSMENT => {
                // chat_data_line_count: u32
                let chat_data_line_count = crate::util::read_u32_le(r)?;

                // chat_data_size_uncompressed: u32
                let chat_data_size_uncompressed = crate::util::read_u32_le(r)?;

                // compressed_chat_data: u8[-]
                let mut current_size = {
                    GmTicketType::size() // category: GmTicketType
                    + Map::size() // map: Map
                    + 4 // position_x: f32
                    + 4 // position_y: f32
                    + 4 // position_z: f32
                    + message.len() + 1 // message: CString and Null Terminator
                    + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString and Null Terminator
                };
                let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                while current_size < (body_size as usize) {
                    compressed_chat_data.push(crate::util::read_u8_le(r)?);
                    current_size += 1;
                }

                CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                    chat_data_line_count,
                    chat_data_size_uncompressed,
                    compressed_chat_data,
                }
            }
            GmTicketType::GUILD => CMSG_GMTICKET_CREATEGmTicketType::GUILD,
            GmTicketType::ITEM => CMSG_GMTICKET_CREATEGmTicketType::ITEM,
            GmTicketType::ENVIRONMENTAL => CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL,
            GmTicketType::NONQUEST_CREEP => CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP,
            GmTicketType::QUEST_QUESTNPC => CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC,
            GmTicketType::TECHNICAL => CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL,
            GmTicketType::ACCOUNT_BILLING => CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING,
            GmTicketType::CHARACTER => CMSG_GMTICKET_CREATEGmTicketType::CHARACTER,
        };

        Ok(Self {
            category: category_if,
            map,
            position_x,
            position_y,
            position_z,
            message,
            reserved_for_future_use,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // category: GmTicketType
        self.category.write(w)?;

        // map: Map
        self.map.write(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // reserved_for_future_use: CString
        w.write_all(self.reserved_for_future_use.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        match &self.category {
            CMSG_GMTICKET_CREATEGmTicketType::STUCK => {}
            CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                chat_data_line_count,
                chat_data_size_uncompressed,
                compressed_chat_data,
            } => {
                // chat_data_line_count: u32
                w.write_all(&chat_data_line_count.to_le_bytes())?;

                // chat_data_size_uncompressed: u32
                w.write_all(&chat_data_size_uncompressed.to_le_bytes())?;

                // compressed_chat_data: u8[-]
                for i in compressed_chat_data.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            CMSG_GMTICKET_CREATEGmTicketType::GUILD => {}
            CMSG_GMTICKET_CREATEGmTicketType::ITEM => {}
            CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL => {}
            CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP => {}
            CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC => {}
            CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL => {}
            CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING => {}
            CMSG_GMTICKET_CREATEGmTicketType::CHARACTER => {}
        }

        Ok(())
    }
}

impl VariableSized for CMSG_GMTICKET_CREATE {
    fn size(&self) -> usize {
        self.category.size() // category: GmTicketType and subfields
        + Map::size() // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + self.message.len() + 1 // message: CString and Null Terminator
        + self.reserved_for_future_use.len() + 1 // reserved_for_future_use: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GMTICKET_CREATE {
    fn maximum_possible_size() -> usize {
        GmTicketType::maximum_possible_size() // category: GmTicketType
        + Map::maximum_possible_size() // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 256 // message: CString
        + 256 // reserved_for_future_use: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GMTICKET_CREATEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GmTicketType(GmTicketTypeError),
    Map(MapError),
}

impl std::error::Error for CMSG_GMTICKET_CREATEError {}
impl std::fmt::Display for CMSG_GMTICKET_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GmTicketType(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GMTICKET_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GMTICKET_CREATEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GmTicketTypeError> for CMSG_GMTICKET_CREATEError {
    fn from(e: GmTicketTypeError) -> Self {
        Self::GmTicketType(e)
    }
}

impl From<MapError> for CMSG_GMTICKET_CREATEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_GMTICKET_CREATEGmTicketType {
    STUCK,
    BEHAVIOR_HARASSMENT {
        chat_data_line_count: u32,
        chat_data_size_uncompressed: u32,
        compressed_chat_data: Vec<u8>,
    },
    GUILD,
    ITEM,
    ENVIRONMENTAL,
    NONQUEST_CREEP,
    QUEST_QUESTNPC,
    TECHNICAL,
    ACCOUNT_BILLING,
    CHARACTER,
}

impl From<&GmTicketType> for CMSG_GMTICKET_CREATEGmTicketType {
    fn from(e: &GmTicketType) -> Self {
        match &e {
            GmTicketType::STUCK => Self::STUCK,
            GmTicketType::BEHAVIOR_HARASSMENT => Self::BEHAVIOR_HARASSMENT {
                chat_data_line_count: Default::default(),
                chat_data_size_uncompressed: Default::default(),
                compressed_chat_data: Default::default(),
            },
            GmTicketType::GUILD => Self::GUILD,
            GmTicketType::ITEM => Self::ITEM,
            GmTicketType::ENVIRONMENTAL => Self::ENVIRONMENTAL,
            GmTicketType::NONQUEST_CREEP => Self::NONQUEST_CREEP,
            GmTicketType::QUEST_QUESTNPC => Self::QUEST_QUESTNPC,
            GmTicketType::TECHNICAL => Self::TECHNICAL,
            GmTicketType::ACCOUNT_BILLING => Self::ACCOUNT_BILLING,
            GmTicketType::CHARACTER => Self::CHARACTER,
        }
    }
}

impl From<&CMSG_GMTICKET_CREATEGmTicketType> for GmTicketType {
    fn from(v: &CMSG_GMTICKET_CREATEGmTicketType) -> Self {
        match &v {
            CMSG_GMTICKET_CREATEGmTicketType::STUCK => Self::STUCK,
            CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT { .. } => Self::BEHAVIOR_HARASSMENT,
            CMSG_GMTICKET_CREATEGmTicketType::GUILD => Self::GUILD,
            CMSG_GMTICKET_CREATEGmTicketType::ITEM => Self::ITEM,
            CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL => Self::ENVIRONMENTAL,
            CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP => Self::NONQUEST_CREEP,
            CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC => Self::QUEST_QUESTNPC,
            CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL => Self::TECHNICAL,
            CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING => Self::ACCOUNT_BILLING,
            CMSG_GMTICKET_CREATEGmTicketType::CHARACTER => Self::CHARACTER,
        }
    }
}

impl Default for CMSG_GMTICKET_CREATEGmTicketType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::STUCK
    }
}

impl CMSG_GMTICKET_CREATEGmTicketType {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketType = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for CMSG_GMTICKET_CREATEGmTicketType {
    fn size(&self) -> usize {
        match self {
            Self::STUCK =>  {
                1
            }
            Self::BEHAVIOR_HARASSMENT  {
                chat_data_line_count,
                chat_data_size_uncompressed,
                compressed_chat_data,
            } => {
                1
                + 4 // chat_data_line_count: u32
                + 4 // chat_data_size_uncompressed: u32
                + compressed_chat_data.len() * core::mem::size_of::<u8>() // compressed_chat_data: u8[-]
            }
            Self::GUILD =>  {
                1
            }
            Self::ITEM =>  {
                1
            }
            Self::ENVIRONMENTAL =>  {
                1
            }
            Self::NONQUEST_CREEP =>  {
                1
            }
            Self::QUEST_QUESTNPC =>  {
                1
            }
            Self::TECHNICAL =>  {
                1
            }
            Self::ACCOUNT_BILLING =>  {
                1
            }
            Self::CHARACTER =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for CMSG_GMTICKET_CREATEGmTicketType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

