use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
#[derive(Copy)]
pub struct CMSG_UNSTABLE_PET {
    pub npc_guid: Guid,
    pub pet_number: u32,
}

impl ClientMessageWrite for CMSG_UNSTABLE_PET {}

impl MessageBody for CMSG_UNSTABLE_PET {
    const OPCODE: u16 = 0x0271;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc_guid,
            pet_number,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.write(w)?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

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
            // npc_guid: Guid
            let npc_guid = Guid::tokio_read(r).await?;

            // pet_number: u32
            let pet_number = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                npc_guid,
                pet_number,
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
            // npc_guid: Guid
            self.npc_guid.tokio_write(w).await?;

            // pet_number: u32
            w.write_all(&self.pet_number.to_le_bytes()).await?;

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
            // npc_guid: Guid
            let npc_guid = Guid::astd_read(r).await?;

            // pet_number: u32
            let pet_number = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                npc_guid,
                pet_number,
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
            // npc_guid: Guid
            self.npc_guid.astd_write(w).await?;

            // pet_number: u32
            w.write_all(&self.pet_number.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_UNSTABLE_PET {}

impl MaximumPossibleSized for CMSG_UNSTABLE_PET {
    fn maximum_possible_size() -> usize {
        0
        + 8 // npc_guid: Guid
        + 4 // pet_number: u32
    }
}

