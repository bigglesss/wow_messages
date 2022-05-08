use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{LogoutResult, LogoutResultError};
use crate::world::v1::v12::{LogoutSpeed, LogoutSpeedError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOGOUT_RESPONSE {
    pub reason: LogoutResult,
    pub speed: LogoutSpeed,
}

impl ServerMessageWrite for SMSG_LOGOUT_RESPONSE {}

impl MessageBody for SMSG_LOGOUT_RESPONSE {
    const OPCODE: u16 = 0x004c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_LOGOUT_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: LogoutResult
        let reason = LogoutResult::read(r)?;

        // speed: LogoutSpeed
        let speed = LogoutSpeed::read(r)?;

        Ok(Self {
            reason,
            speed,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: LogoutResult
        self.reason.write(w)?;

        // speed: LogoutSpeed
        self.speed.write(w)?;

        Ok(())
    }

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
            // reason: LogoutResult
            let reason = LogoutResult::tokio_read(r).await?;

            // speed: LogoutSpeed
            let speed = LogoutSpeed::tokio_read(r).await?;

            Ok(Self {
                reason,
                speed,
            })
        })
    }

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
            // reason: LogoutResult
            self.reason.tokio_write(w).await?;

            // speed: LogoutSpeed
            self.speed.tokio_write(w).await?;

            Ok(())
        })
    }

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
            // reason: LogoutResult
            let reason = LogoutResult::astd_read(r).await?;

            // speed: LogoutSpeed
            let speed = LogoutSpeed::astd_read(r).await?;

            Ok(Self {
                reason,
                speed,
            })
        })
    }

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
            // reason: LogoutResult
            self.reason.astd_write(w).await?;

            // speed: LogoutSpeed
            self.speed.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_LOGOUT_RESPONSE {}

impl MaximumPossibleSized for SMSG_LOGOUT_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // reason: LogoutResult
        + 1 // speed: LogoutSpeed
    }
}

#[derive(Debug)]
pub enum SMSG_LOGOUT_RESPONSEError {
    Io(std::io::Error),
    LogoutResult(LogoutResultError),
    LogoutSpeed(LogoutSpeedError),
}

impl std::error::Error for SMSG_LOGOUT_RESPONSEError {}
impl std::fmt::Display for SMSG_LOGOUT_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LogoutResult(i) => i.fmt(f),
            Self::LogoutSpeed(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOGOUT_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LogoutResultError> for SMSG_LOGOUT_RESPONSEError {
    fn from(e: LogoutResultError) -> Self {
        Self::LogoutResult(e)
    }
}

impl From<LogoutSpeedError> for SMSG_LOGOUT_RESPONSEError {
    fn from(e: LogoutSpeedError) -> Self {
        Self::LogoutSpeed(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::SMSG_LOGOUT_RESPONSE;
    use crate::ConstantSized;
    use crate::world::v1::v12::LogoutResult;
    use crate::world::v1::v12::LogoutSpeed;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(SMSG_LOGOUT_RESPONSE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(SMSG_LOGOUT_RESPONSE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(SMSG_LOGOUT_RESPONSE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
