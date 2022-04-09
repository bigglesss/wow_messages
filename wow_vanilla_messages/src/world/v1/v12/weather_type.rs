use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L3):
/// ```text
/// enum WeatherType : u32 {
///     FINE = 0;
///     RAIN = 1;
///     SNOW = 2;
///     STORM = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WeatherType {
    FINE,
    RAIN,
    SNOW,
    STORM,
}

impl ReadableAndWritable for WeatherType {
    type Error = WeatherTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl WeatherType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::FINE => 0x0,
            Self::RAIN => 0x1,
            Self::SNOW => 0x2,
            Self::STORM => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::FINE
    }

}

impl ConstantSized for WeatherType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for WeatherType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for WeatherType {
    fn default() -> Self {
        Self::FINE
    }
}

impl std::fmt::Display for WeatherType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FINE => f.write_str("FINE"),
            Self::RAIN => f.write_str("RAIN"),
            Self::SNOW => f.write_str("SNOW"),
            Self::STORM => f.write_str("STORM"),
        }
    }
}

impl TryFrom<u32> for WeatherType {
    type Error = TryFromWeatherTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FINE),
            1 => Ok(Self::RAIN),
            2 => Ok(Self::SNOW),
            3 => Ok(Self::STORM),
            _ => Err(TryFromWeatherTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromWeatherTypeError {
    value: u32,
}

impl TryFromWeatherTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum WeatherTypeError {
    Read(std::io::Error),
    TryFrom(TryFromWeatherTypeError),
}

impl std::error::Error for WeatherTypeError {}
impl std::fmt::Display for TryFromWeatherTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'WeatherType': '{}'", self.value))
    }
}

impl std::fmt::Display for WeatherTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for WeatherTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromWeatherTypeError> for WeatherTypeError {
    fn from(value: TryFromWeatherTypeError) -> Self {
        Self::TryFrom(value)
    }
}
