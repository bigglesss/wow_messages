use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Character, CharacterError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CHAR_ENUM {
    pub characters: Vec<Character>,
}

impl ServerMessageWrite for SMSG_CHAR_ENUM {}

impl MessageBody for SMSG_CHAR_ENUM {
    const OPCODE: u16 = 0x003b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CHAR_ENUMError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_characters: u8
        let amount_of_characters = crate::util::read_u8_le(r)?;

        // characters: Character[amount_of_characters]
        let mut characters = Vec::with_capacity(amount_of_characters as usize);
        for i in 0..amount_of_characters {
            characters.push(Character::read(r)?);
        }

        Ok(Self {
            characters,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            i.write(w)?;
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
            // amount_of_characters: u8
            let amount_of_characters = crate::util::tokio_read_u8_le(r).await?;

            // characters: Character[amount_of_characters]
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for i in 0..amount_of_characters {
                characters.push(Character::tokio_read(r).await?);
            }

            Ok(Self {
                characters,
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
            // amount_of_characters: u8
            w.write_all(&(self.characters.len() as u8).to_le_bytes()).await?;

            // characters: Character[amount_of_characters]
            for i in self.characters.iter() {
                i.tokio_write(w).await?;
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
            // amount_of_characters: u8
            let amount_of_characters = crate::util::astd_read_u8_le(r).await?;

            // characters: Character[amount_of_characters]
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for i in 0..amount_of_characters {
                characters.push(Character::astd_read(r).await?);
            }

            Ok(Self {
                characters,
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
            // amount_of_characters: u8
            w.write_all(&(self.characters.len() as u8).to_le_bytes()).await?;

            // characters: Character[amount_of_characters]
            for i in self.characters.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_CHAR_ENUM {
    fn size(&self) -> usize {
        0
        + 1 // amount_of_characters: u8
        + self.characters.iter().fold(0, |acc, x| acc + x.size()) // characters: Character[amount_of_characters]
    }
}

impl MaximumPossibleSized for SMSG_CHAR_ENUM {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum SMSG_CHAR_ENUMError {
    Io(std::io::Error),
    Character(CharacterError),
}

impl std::error::Error for SMSG_CHAR_ENUMError {}
impl std::fmt::Display for SMSG_CHAR_ENUMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Character(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHAR_ENUMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CharacterError> for SMSG_CHAR_ENUMError {
    fn from(e: CharacterError) -> Self {
        Self::Character(e)
    }
}

