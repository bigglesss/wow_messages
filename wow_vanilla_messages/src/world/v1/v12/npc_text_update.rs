use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Language, LanguageError};
use crate::world::v1::v12::NpcTextUpdateEmote;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct NpcTextUpdate {
    pub probability: f32,
    pub texts: [String; 2],
    pub language: Language,
    pub emotes: [NpcTextUpdateEmote; 3],
}

impl ReadableAndWritable for NpcTextUpdate {
    type Error = NpcTextUpdateError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: CString[2]
        let mut texts = Vec::with_capacity(2 as usize);
        for i in 0..2 {
            let s = crate::util::read_c_string_to_vec(r)?;
            texts[i] = String::from_utf8(s)?;
        }
        let texts = texts.try_into().unwrap();

        // language: Language
        let language = Language::read(r)?;

        // emotes: NpcTextUpdateEmote[3]
        let mut emotes = Vec::with_capacity(3 as usize);
        for i in 0..3 {
            emotes.push(NpcTextUpdateEmote::read(r)?);
        }
        let emotes = emotes.try_into().unwrap();

        Ok(Self {
            probability,
            texts,
            language,
            emotes,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: CString[2]
        for i in self.texts.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // language: Language
        self.language.write(w)?;

        // emotes: NpcTextUpdateEmote[3]
        for i in self.emotes.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // probability: f32
            let probability = crate::util::tokio_read_f32_le(r).await?;
            // texts: CString[2]
            let mut texts = Vec::with_capacity(2 as usize);
            for i in 0..2 {
                let s = crate::util::tokio_read_c_string_to_vec(r).await?;
                texts[i] = String::from_utf8(s)?;
            }
            let texts = texts.try_into().unwrap();

            // language: Language
            let language = Language::tokio_read(r).await?;

            // emotes: NpcTextUpdateEmote[3]
            let mut emotes = Vec::with_capacity(3 as usize);
            for i in 0..3 {
                emotes.push(NpcTextUpdateEmote::tokio_read(r).await?);
            }
            let emotes = emotes.try_into().unwrap();

            Ok(Self {
                probability,
                texts,
                language,
                emotes,
            })
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            // probability: f32
            w.write_all(&self.probability.to_le_bytes()).await?;

            // texts: CString[2]
            for i in self.texts.iter() {
                w.write_all(&i.as_bytes()).await?;
                w.write_all(&[0]).await?;
            }

            // language: Language
            self.language.tokio_write(w).await?;

            // emotes: NpcTextUpdateEmote[3]
            for i in self.emotes.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // probability: f32
            let probability = crate::util::astd_read_f32_le(r).await?;
            // texts: CString[2]
            let mut texts = Vec::with_capacity(2 as usize);
            for i in 0..2 {
                let s = crate::util::astd_read_c_string_to_vec(r).await?;
                texts[i] = String::from_utf8(s)?;
            }
            let texts = texts.try_into().unwrap();

            // language: Language
            let language = Language::astd_read(r).await?;

            // emotes: NpcTextUpdateEmote[3]
            let mut emotes = Vec::with_capacity(3 as usize);
            for i in 0..3 {
                emotes.push(NpcTextUpdateEmote::astd_read(r).await?);
            }
            let emotes = emotes.try_into().unwrap();

            Ok(Self {
                probability,
                texts,
                language,
                emotes,
            })
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            // probability: f32
            w.write_all(&self.probability.to_le_bytes()).await?;

            // texts: CString[2]
            for i in self.texts.iter() {
                w.write_all(&i.as_bytes()).await?;
                w.write_all(&[0]).await?;
            }

            // language: Language
            self.language.astd_write(w).await?;

            // emotes: NpcTextUpdateEmote[3]
            for i in self.emotes.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for NpcTextUpdate {
    fn size(&self) -> usize {
        0
        + 4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.len() + 1) // texts: CString[2]
        + 4 // language: Language
        + 3 * NpcTextUpdateEmote::size() // emotes: NpcTextUpdateEmote[3]
    }
}

impl MaximumPossibleSized for NpcTextUpdate {
    fn maximum_possible_size() -> usize {
        0
        + 4 // probability: f32
        + 512 // texts: CString[2]
        + 4 // language: Language
        + 24 // emotes: NpcTextUpdateEmote[3]
    }
}

#[derive(Debug)]
pub enum NpcTextUpdateError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Language(LanguageError),
}

impl std::error::Error for NpcTextUpdateError {}
impl std::fmt::Display for NpcTextUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Language(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for NpcTextUpdateError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for NpcTextUpdateError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<LanguageError> for NpcTextUpdateError {
    fn from(e: LanguageError) -> Self {
        Self::Language(e)
    }
}

