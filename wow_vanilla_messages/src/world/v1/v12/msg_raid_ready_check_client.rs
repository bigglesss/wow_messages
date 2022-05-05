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
pub struct MSG_RAID_READY_CHECK_Client {
    pub answer: Option<MSG_RAID_READY_CHECK_Client_answer>,
}

impl ClientMessageWrite for MSG_RAID_READY_CHECK_Client {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_RAID_READY_CHECK_Client {
    const OPCODE: u16 = 0x0322;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional answer
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
        };
        let answer = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Client_answer {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            answer,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional answer
        if let Some(v) = &self.answer {
            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional answer
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
        };
        let answer = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::tokio_read_u8_le(r).await?;

            Some(MSG_RAID_READY_CHECK_Client_answer {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            answer,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional answer
        if let Some(v) = &self.answer {
            // state: u8
            w.write_all(&v.state.to_le_bytes()).await?;

        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional answer
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
        };
        let answer = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::astd_read_u8_le(r).await?;

            Some(MSG_RAID_READY_CHECK_Client_answer {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            answer,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional answer
        if let Some(v) = &self.answer {
            // state: u8
            w.write_all(&v.state.to_le_bytes()).await?;

        }

        Ok(())
    }

}

impl VariableSized for MSG_RAID_READY_CHECK_Client {
    fn size(&self) -> usize {
        {
            if let Some(v) = &self.answer {
                v.size()
            } else {
                0
            }
        } // optional answer
    }
}

impl MaximumPossibleSized for MSG_RAID_READY_CHECK_Client {
    fn maximum_possible_size() -> usize {
        65536 // optional answer
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Client_answer {
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Client_answer {
    pub fn size(&self) -> usize {
        1 // state: u8
    }
}

