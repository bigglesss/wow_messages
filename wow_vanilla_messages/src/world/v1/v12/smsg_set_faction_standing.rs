use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Faction;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SET_FACTION_STANDING {
    pub factions: Vec<Faction>,
}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_SET_FACTION_STANDING {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
    const OPCODE: u16 = 0x0124;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: Faction[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(Faction::read(r)?);
        }

        Ok(Self {
            factions,
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
            // amount_of_factions: u32
            let amount_of_factions = crate::util::tokio_read_u32_le(r).await?;

            // factions: Faction[amount_of_factions]
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for i in 0..amount_of_factions {
                factions.push(Faction::tokio_read(r).await?);
            }

            Ok(Self {
                factions,
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
            // amount_of_factions: u32
            let amount_of_factions = crate::util::astd_read_u32_le(r).await?;

            // factions: Faction[amount_of_factions]
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for i in 0..amount_of_factions {
                factions.push(Faction::astd_read(r).await?);
            }

            Ok(Self {
                factions,
            })
        })
    }

}

impl SMSG_SET_FACTION_STANDING {
    pub fn size(&self) -> usize {
        0
        + 4 // amount_of_factions: u32
        + self.factions.len() * 8 // factions: Faction[amount_of_factions]
    }
}

