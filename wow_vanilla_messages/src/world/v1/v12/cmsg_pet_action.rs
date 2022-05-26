use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PET_ACTION {
    pub pet_guid: Guid,
    pub data: u32,
    pub target_guid: Guid,
}

impl CMSG_PET_ACTION {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 20], std::io::Error> {
        let mut array_w = [0u8; 20];
        let mut w = array_w.as_mut_slice();
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_PET_ACTION {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0175;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
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
            // pet_guid: Guid
            let pet_guid = Guid::tokio_read(r).await?;

            // data: u32
            let data = crate::util::tokio_read_u32_le(r).await?;

            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                pet_guid,
                data,
                target_guid,
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
            // pet_guid: Guid
            let pet_guid = Guid::astd_read(r).await?;

            // data: u32
            let data = crate::util::astd_read_u32_le(r).await?;

            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            Ok(Self {
                pet_guid,
                data,
                target_guid,
            })
        })
    }

}

