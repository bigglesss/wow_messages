use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_GMTICKET_SYSTEMSTATUS {
    pub will_accept_tickets: u32,
}

impl ServerMessageWrite for SMSG_GMTICKET_SYSTEMSTATUS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GMTICKET_SYSTEMSTATUS {
    const OPCODE: u16 = 0x021b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // will_accept_tickets: u32
        let will_accept_tickets = crate::util::read_u32_le(r)?;

        Ok(Self {
            will_accept_tickets,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // will_accept_tickets: u32
        w.write_all(&self.will_accept_tickets.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // will_accept_tickets: u32
        let will_accept_tickets = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            will_accept_tickets,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // will_accept_tickets: u32
        w.write_all(&self.will_accept_tickets.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // will_accept_tickets: u32
        let will_accept_tickets = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            will_accept_tickets,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // will_accept_tickets: u32
        w.write_all(&self.will_accept_tickets.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_GMTICKET_SYSTEMSTATUS {}

impl MaximumPossibleSized for SMSG_GMTICKET_SYSTEMSTATUS {
    fn maximum_possible_size() -> usize {
        4 // will_accept_tickets: u32
    }
}

