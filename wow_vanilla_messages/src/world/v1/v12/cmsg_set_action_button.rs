use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    pub action_type: u32,
}

impl ClientMessageWrite for CMSG_SET_ACTION_BUTTON {}

impl MessageBody for CMSG_SET_ACTION_BUTTON {
    const OPCODE: u16 = 0x0128;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // button: u8
        let button = crate::util::read_u8_le(r)?;

        // action_type: u32
        let action_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            button,
            action_type,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action_type: u32
        w.write_all(&self.action_type.to_le_bytes())?;

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
            // button: u8
            let button = crate::util::tokio_read_u8_le(r).await?;

            // action_type: u32
            let action_type = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                button,
                action_type,
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
            // button: u8
            w.write_all(&self.button.to_le_bytes()).await?;

            // action_type: u32
            w.write_all(&self.action_type.to_le_bytes()).await?;

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
            // button: u8
            let button = crate::util::astd_read_u8_le(r).await?;

            // action_type: u32
            let action_type = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                button,
                action_type,
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
            // button: u8
            w.write_all(&self.button.to_le_bytes()).await?;

            // action_type: u32
            w.write_all(&self.action_type.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_SET_ACTION_BUTTON {}

impl MaximumPossibleSized for CMSG_SET_ACTION_BUTTON {
    fn maximum_possible_size() -> usize {
        0
        + 1 // button: u8
        + 4 // action_type: u32
    }
}

