use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetFeedback {
    PET_DEAD,
    NOTHING_TO_EAT,
    CANT_ATTACK_TARGET,
    NO_PATH_TO,
}

impl PetFeedback {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PET_DEAD => 0x1,
            Self::NOTHING_TO_EAT => 0x2,
            Self::CANT_ATTACK_TARGET => 0x3,
            Self::NO_PATH_TO => 0x4,
        }
    }

}

impl ConstantSized for PetFeedback {}

impl MaximumPossibleSized for PetFeedback {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PetFeedback {
    fn default() -> Self {
        Self::PET_DEAD
    }
}

impl std::fmt::Display for PetFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PET_DEAD => f.write_str("PET_DEAD"),
            Self::NOTHING_TO_EAT => f.write_str("NOTHING_TO_EAT"),
            Self::CANT_ATTACK_TARGET => f.write_str("CANT_ATTACK_TARGET"),
            Self::NO_PATH_TO => f.write_str("NO_PATH_TO"),
        }
    }
}

impl TryFrom<u8> for PetFeedback {
    type Error = PetFeedbackError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PET_DEAD),
            2 => Ok(Self::NOTHING_TO_EAT),
            3 => Ok(Self::CANT_ATTACK_TARGET),
            4 => Ok(Self::NO_PATH_TO),
            _ => Err(PetFeedbackError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PetFeedbackError {
    value: u8,
}

impl PetFeedbackError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PetFeedbackError {}
impl std::fmt::Display for PetFeedbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetFeedback': '{}'", self.value))
    }
}

