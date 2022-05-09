use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Serverstate_check>,
}

impl ServerMessageWrite for MSG_RAID_READY_CHECK_Server {}

impl MessageBody for MSG_RAID_READY_CHECK_Server {
    const OPCODE: u16 = 0x0322;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional state_check
        let current_size = {
            0
        };
        let state_check = if current_size < body_size as usize {
            // guid: Guid
            let guid = Guid::read(r)?;

            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Serverstate_check {
                guid,
                state,
            })
        } else {
            None
        };

        Ok(Self {
            state_check,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional state_check
        if let Some(v) = &self.state_check {
            // guid: Guid
            v.guid.write(w)?;

            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

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
            // optional state_check
            let current_size = {
                0
            };
            let state_check = if current_size < body_size as usize {
                // guid: Guid
                let guid = Guid::tokio_read(r).await?;

                // state: u8
                let state = crate::util::tokio_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Serverstate_check {
                    guid,
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                state_check,
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
            // optional state_check
            if let Some(v) = &self.state_check {
                // guid: Guid
                v.guid.tokio_write(w).await?;

                // state: u8
                w.write_all(&v.state.to_le_bytes()).await?;

            }

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
            // optional state_check
            let current_size = {
                0
            };
            let state_check = if current_size < body_size as usize {
                // guid: Guid
                let guid = Guid::astd_read(r).await?;

                // state: u8
                let state = crate::util::astd_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Serverstate_check {
                    guid,
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                state_check,
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
            // optional state_check
            if let Some(v) = &self.state_check {
                // guid: Guid
                v.guid.astd_write(w).await?;

                // state: u8
                w.write_all(&v.state.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for MSG_RAID_READY_CHECK_Server {
    fn size(&self) -> usize {
        0
        + if let Some(state_check) = &self.state_check {
            0
            + 8 // guid: Guid
            + 1 // state: u8
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for MSG_RAID_READY_CHECK_Server {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Serverstate_check {
    pub guid: Guid,
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Serverstate_check {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // state: u8
    }

}

