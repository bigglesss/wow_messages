use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WhoPlayer {
    pub name: String,
    pub guild: String,
    pub level: u32,
    pub class: Class,
    pub race: Race,
    pub zone_id: u32,
    pub party_status: u32,
}

impl ReadableAndWritable for WhoPlayer {
    type Error = WhoPlayerError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guild: CString
        let guild = crate::util::read_c_string_to_vec(r)?;
        let guild = String::from_utf8(guild)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // class: Class
        let class = Class::read(r)?;

        // race: Race
        let race = Race::read(r)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // party_status: u32
        let party_status = crate::util::read_u32_le(r)?;

        Ok(Self {
            name,
            guild,
            level,
            class,
            race,
            zone_id,
            party_status,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild: CString
        w.write_all(self.guild.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // class: Class
        self.class.write(w)?;

        // race: Race
        self.race.write(w)?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // party_status: u32
        w.write_all(&self.party_status.to_le_bytes())?;

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
            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // guild: CString
            let guild = crate::util::tokio_read_c_string_to_vec(r).await?;
            let guild = String::from_utf8(guild)?;

            // level: u32
            let level = crate::util::tokio_read_u32_le(r).await?;

            // class: Class
            let class = Class::tokio_read(r).await?;

            // race: Race
            let race = Race::tokio_read(r).await?;

            // zone_id: u32
            let zone_id = crate::util::tokio_read_u32_le(r).await?;

            // party_status: u32
            let party_status = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                name,
                guild,
                level,
                class,
                race,
                zone_id,
                party_status,
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
            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // guild: CString
            w.write_all(self.guild.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // level: u32
            w.write_all(&self.level.to_le_bytes()).await?;

            // class: Class
            self.class.tokio_write(w).await?;

            // race: Race
            self.race.tokio_write(w).await?;

            // zone_id: u32
            w.write_all(&self.zone_id.to_le_bytes()).await?;

            // party_status: u32
            w.write_all(&self.party_status.to_le_bytes()).await?;

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
            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // guild: CString
            let guild = crate::util::astd_read_c_string_to_vec(r).await?;
            let guild = String::from_utf8(guild)?;

            // level: u32
            let level = crate::util::astd_read_u32_le(r).await?;

            // class: Class
            let class = Class::astd_read(r).await?;

            // race: Race
            let race = Race::astd_read(r).await?;

            // zone_id: u32
            let zone_id = crate::util::astd_read_u32_le(r).await?;

            // party_status: u32
            let party_status = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                name,
                guild,
                level,
                class,
                race,
                zone_id,
                party_status,
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
            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // guild: CString
            w.write_all(self.guild.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // level: u32
            w.write_all(&self.level.to_le_bytes()).await?;

            // class: Class
            self.class.astd_write(w).await?;

            // race: Race
            self.race.astd_write(w).await?;

            // zone_id: u32
            w.write_all(&self.zone_id.to_le_bytes()).await?;

            // party_status: u32
            w.write_all(&self.party_status.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for WhoPlayer {
    fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + self.guild.len() + 1 // guild: CString
        + 4 // level: u32
        + 1 // class: Class
        + 1 // race: Race
        + 4 // zone_id: u32
        + 4 // party_status: u32
    }
}

impl MaximumPossibleSized for WhoPlayer {
    fn maximum_possible_size() -> usize {
        0
        + 256 // name: CString
        + 256 // guild: CString
        + 4 // level: u32
        + 1 // class: Class
        + 1 // race: Race
        + 4 // zone_id: u32
        + 4 // party_status: u32
    }
}

#[derive(Debug)]
pub enum WhoPlayerError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Class(ClassError),
    Race(RaceError),
}

impl std::error::Error for WhoPlayerError {}
impl std::fmt::Display for WhoPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for WhoPlayerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for WhoPlayerError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ClassError> for WhoPlayerError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<RaceError> for WhoPlayerError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

