use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SHOWTAXINODES {
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl ServerMessageWrite for SMSG_SHOWTAXINODES {}

impl MessageBody for SMSG_SHOWTAXINODES {
    const OPCODE: u16 = 0x01a9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(r)?;

        // nodes: u32[-]
        let mut current_size = {
            4 // unknown1: u32
            + 8 // guid: Guid
            + 4 // nearest_node: u32
        };
        let mut nodes = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            nodes.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
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
            // unknown1: u32
            let unknown1 = crate::util::tokio_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // nearest_node: u32
            let nearest_node = crate::util::tokio_read_u32_le(r).await?;

            // nodes: u32[-]
            let mut current_size = {
                4 // unknown1: u32
                + 8 // guid: Guid
                + 4 // nearest_node: u32
            };
            let mut nodes = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                nodes.push(crate::util::tokio_read_u32_le(r).await?);
                current_size += 1;
            }

            Ok(Self {
                unknown1,
                guid,
                nearest_node,
                nodes,
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
            // unknown1: u32
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // guid: Guid
            self.guid.tokio_write(w).await?;

            // nearest_node: u32
            w.write_all(&self.nearest_node.to_le_bytes()).await?;

            // nodes: u32[-]
            for i in self.nodes.iter() {
                w.write_all(&i.to_le_bytes()).await?;
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
            // unknown1: u32
            let unknown1 = crate::util::astd_read_u32_le(r).await?;

            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // nearest_node: u32
            let nearest_node = crate::util::astd_read_u32_le(r).await?;

            // nodes: u32[-]
            let mut current_size = {
                4 // unknown1: u32
                + 8 // guid: Guid
                + 4 // nearest_node: u32
            };
            let mut nodes = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                nodes.push(crate::util::astd_read_u32_le(r).await?);
                current_size += 1;
            }

            Ok(Self {
                unknown1,
                guid,
                nearest_node,
                nodes,
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
            // unknown1: u32
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // guid: Guid
            self.guid.astd_write(w).await?;

            // nearest_node: u32
            w.write_all(&self.nearest_node.to_le_bytes()).await?;

            // nodes: u32[-]
            for i in self.nodes.iter() {
                w.write_all(&i.to_le_bytes()).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_SHOWTAXINODES {
    fn size(&self) -> usize {
        0
        + 4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_SHOWTAXINODES {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

