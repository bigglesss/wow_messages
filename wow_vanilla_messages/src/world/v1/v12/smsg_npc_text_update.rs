use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::NpcTextUpdate;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub probability: f32,
    pub texts: [NpcTextUpdate; 8],
}

impl SMSG_NPC_TEXT_UPDATE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_NPC_TEXT_UPDATE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0180;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: NpcTextUpdate[8]
        let mut texts = Vec::with_capacity(8 as usize);
        for i in 0..8 {
            texts.push(NpcTextUpdate::read(r)?);
        }
        let texts = texts.try_into().unwrap();

        Ok(Self {
            text_id,
            probability,
            texts,
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
            // text_id: u32
            let text_id = crate::util::tokio_read_u32_le(r).await?;

            // probability: f32
            let probability = crate::util::tokio_read_f32_le(r).await?;
            // texts: NpcTextUpdate[8]
            let mut texts = Vec::with_capacity(8 as usize);
            for i in 0..8 {
                texts.push(NpcTextUpdate::tokio_read(r).await?);
            }
            let texts = texts.try_into().unwrap();

            Ok(Self {
                text_id,
                probability,
                texts,
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
            // text_id: u32
            let text_id = crate::util::astd_read_u32_le(r).await?;

            // probability: f32
            let probability = crate::util::astd_read_f32_le(r).await?;
            // texts: NpcTextUpdate[8]
            let mut texts = Vec::with_capacity(8 as usize);
            for i in 0..8 {
                texts.push(NpcTextUpdate::astd_read(r).await?);
            }
            let texts = texts.try_into().unwrap();

            Ok(Self {
                text_id,
                probability,
                texts,
            })
        })
    }

}

impl SMSG_NPC_TEXT_UPDATE {
    pub fn size(&self) -> usize {
        0
        + 4 // text_id: u32
        + 4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

