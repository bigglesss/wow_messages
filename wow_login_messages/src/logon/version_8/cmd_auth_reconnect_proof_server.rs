use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_AUTH_RECONNECT_PROOF_Server {
    pub result: LoginResult,
}

impl ServerMessage for CMD_AUTH_RECONNECT_PROOF_Server {
    const OPCODE: u8 = 0x03;
}
impl CMD_AUTH_RECONNECT_PROOF_Server {
    pub const PADDING_VALUE: u16 = 0x00;

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CMD_AUTH_RECONNECT_PROOF_Server {
    type Error = CMD_AUTH_RECONNECT_PROOF_ServerError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result = LoginResult::read(r)?;

        // padding: u16
        let _padding = crate::util::read_u16_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        self.result.write(w)?;

        // padding: u16
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result = LoginResult::tokio_read(r).await?;

        // padding: u16
        let _padding = crate::util::tokio_read_u16_le(r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // result: LoginResult
        self.result.tokio_write(w).await?;

        // padding: u16
        w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result = LoginResult::astd_read(r).await?;

        // padding: u16
        let _padding = crate::util::astd_read_u16_le(r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // result: LoginResult
        self.result.astd_write(w).await?;

        // padding: u16
        w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMD_AUTH_RECONNECT_PROOF_Server {}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_PROOF_Server {
    fn maximum_possible_size() -> usize {
        LoginResult::size() // result: LoginResult
        + 2 // padding: u16
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_RECONNECT_PROOF_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_RECONNECT_PROOF_ServerError {}
impl std::fmt::Display for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_RECONNECT_PROOF_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::ConstantSized;
    use crate::logon::version_8::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    #[test]
    #[cfg(feature = "sync")]
    fn CMD_AUTH_RECONNECT_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x03, 0x00, 0x00, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::SUCCESS,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Server::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    #[cfg(feature = "sync")]
    fn CMD_AUTH_RECONNECT_PROOF_Server1() {
        let raw: Vec<u8> = vec![ 0x03, 0x10, 0x00, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::FAIL_LOCKED_ENFORCED,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Server::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
