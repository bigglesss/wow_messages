use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GuildEvent;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_EVENT {
    pub event: GuildEvent,
    pub event_descriptions: Vec<String>,
}

impl SMSG_GUILD_EVENT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // event: GuildEvent
        w.write_all(&(self.event.as_int() as u8).to_le_bytes())?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_GUILD_EVENT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // event: GuildEvent
        w.write_all(&(self.event.as_int() as u8).to_le_bytes())?;

        // amount_of_events: u8
        w.write_all(&(self.event_descriptions.len() as u8).to_le_bytes())?;

        // event_descriptions: CString[amount_of_events]
        for i in self.event_descriptions.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0092;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // event: GuildEvent
        let event: GuildEvent = crate::util::read_u8_le(r)?.try_into()?;

        // amount_of_events: u8
        let amount_of_events = crate::util::read_u8_le(r)?;

        // event_descriptions: CString[amount_of_events]
        let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
        for i in 0..amount_of_events {
            let s = crate::util::read_c_string_to_vec(r)?;
            event_descriptions.push(String::from_utf8(s)?);
        }

        Ok(Self {
            event,
            event_descriptions,
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
            // event: GuildEvent
            let event: GuildEvent = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // amount_of_events: u8
            let amount_of_events = crate::util::tokio_read_u8_le(r).await?;

            // event_descriptions: CString[amount_of_events]
            let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
            for i in 0..amount_of_events {
                let s = crate::util::tokio_read_c_string_to_vec(r).await?;
                event_descriptions.push(String::from_utf8(s)?);
            }

            Ok(Self {
                event,
                event_descriptions,
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
            // event: GuildEvent
            let event: GuildEvent = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // amount_of_events: u8
            let amount_of_events = crate::util::astd_read_u8_le(r).await?;

            // event_descriptions: CString[amount_of_events]
            let mut event_descriptions = Vec::with_capacity(amount_of_events as usize);
            for i in 0..amount_of_events {
                let s = crate::util::astd_read_c_string_to_vec(r).await?;
                event_descriptions.push(String::from_utf8(s)?);
            }

            Ok(Self {
                event,
                event_descriptions,
            })
        })
    }

}

impl SMSG_GUILD_EVENT {
    pub fn size(&self) -> usize {
        0
        + 1 // event: GuildEvent
        + 1 // amount_of_events: u8
        + self.event_descriptions.iter().fold(0, |acc, x| acc + x.len() + 1) // event_descriptions: CString[amount_of_events]
    }
}

