use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::DuelWinnerReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_DUEL_WINNER {
    pub reason: DuelWinnerReason,
    pub opponent_name: String,
    pub initiator_name: String,
}

impl SMSG_DUEL_WINNER {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // reason: DuelWinnerReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // opponent_name: CString
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_DUEL_WINNER {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: DuelWinnerReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // opponent_name: CString
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x016b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_DUEL_WINNERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: DuelWinnerReason
        let reason: DuelWinnerReason = crate::util::read_u8_le(r)?.try_into()?;

        // opponent_name: CString
        let opponent_name = crate::util::read_c_string_to_vec(r)?;
        let opponent_name = String::from_utf8(opponent_name)?;

        // initiator_name: CString
        let initiator_name = crate::util::read_c_string_to_vec(r)?;
        let initiator_name = String::from_utf8(initiator_name)?;

        Ok(Self {
            reason,
            opponent_name,
            initiator_name,
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
            // reason: DuelWinnerReason
            let reason: DuelWinnerReason = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // opponent_name: CString
            let opponent_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let opponent_name = String::from_utf8(opponent_name)?;

            // initiator_name: CString
            let initiator_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let initiator_name = String::from_utf8(initiator_name)?;

            Ok(Self {
                reason,
                opponent_name,
                initiator_name,
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
            // reason: DuelWinnerReason
            let reason: DuelWinnerReason = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // opponent_name: CString
            let opponent_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let opponent_name = String::from_utf8(opponent_name)?;

            // initiator_name: CString
            let initiator_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let initiator_name = String::from_utf8(initiator_name)?;

            Ok(Self {
                reason,
                opponent_name,
                initiator_name,
            })
        })
    }

}

impl SMSG_DUEL_WINNER {
    pub fn size(&self) -> usize {
        0
        + 1 // reason: DuelWinnerReason
        + self.opponent_name.len() + 1 // opponent_name: CString
        + self.initiator_name.len() + 1 // initiator_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_DUEL_WINNERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_DUEL_WINNERError {}
impl std::fmt::Display for SMSG_DUEL_WINNERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_DUEL_WINNERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_DUEL_WINNERError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_DUEL_WINNERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

