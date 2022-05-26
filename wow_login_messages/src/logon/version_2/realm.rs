use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Population;
use crate::logon::version_2::{RealmCategory, RealmCategoryError};
use crate::logon::version_2::{RealmFlag};
use crate::logon::version_2::{RealmType, RealmTypeError};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Realm {
    pub realm_type: RealmType,
    pub flag: RealmFlag,
    pub name: String,
    pub address: String,
    pub population: Population,
    pub number_of_characters_on_realm: u8,
    pub category: RealmCategory,
    pub realm_id: u8,
}

impl Realm {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(521);
        // realm_type: RealmType
        w.write_all(&(self.realm_type.as_int() as u32).to_le_bytes())?;

        // flag: RealmFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        w.write_all(&(self.population.as_int() as u32).to_le_bytes())?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        w.write_all(&(self.category.as_int() as u8).to_le_bytes())?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        Ok(w)
    }
}

impl Realm {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmError> {
        // realm_type: RealmType
        let realm_type: RealmType = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::read_u8_le(r)?);

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::read_c_string_to_vec(r)?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::read_u32_le(r)?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::read_u8_le(r)?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::read_u8_le(r)?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::read_u8_le(r)?;

        Ok(Self {
            realm_type,
            flag,
            name,
            address,
            population,
            number_of_characters_on_realm,
            category,
            realm_id,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RealmError> {
        // realm_type: RealmType
        let realm_type: RealmType = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::tokio_read_u8_le(r).await?);

        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::tokio_read_c_string_to_vec(r).await?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::tokio_read_u32_le(r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::tokio_read_u8_le(r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            realm_type,
            flag,
            name,
            address,
            population,
            number_of_characters_on_realm,
            category,
            realm_id,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RealmError> {
        // realm_type: RealmType
        let realm_type: RealmType = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::astd_read_u8_le(r).await?);

        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::astd_read_c_string_to_vec(r).await?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::astd_read_u32_le(r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::astd_read_u8_le(r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::astd_read_u8_le(r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            realm_type,
            flag,
            name,
            address,
            population,
            number_of_characters_on_realm,
            category,
            realm_id,
        })
    }

}

impl Realm {
    pub fn size(&self) -> usize {
        0
        + 4 // realm_type: RealmType
        + 1 // flag: RealmFlag
        + self.name.len() + 1 // name: CString
        + self.address.len() + 1 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
        + 1 // realm_id: u8
    }
}

#[derive(Debug)]
pub enum RealmError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    RealmCategory(RealmCategoryError),
    RealmType(RealmTypeError),
}

impl std::error::Error for RealmError {}
impl std::fmt::Display for RealmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::RealmCategory(i) => i.fmt(f),
            Self::RealmType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RealmError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for RealmError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<RealmCategoryError> for RealmError {
    fn from(e: RealmCategoryError) -> Self {
        Self::RealmCategory(e)
    }
}

impl From<RealmTypeError> for RealmError {
    fn from(e: RealmTypeError) -> Self {
        Self::RealmType(e)
    }
}

