use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/client.wowm#L3):
/// ```text
/// clogin CMD_REALM_LIST_Client = 0x10 {
///     u32 padding = 0;
/// }
/// ```
pub struct CMD_REALM_LIST_Client {
}

impl CMD_REALM_LIST_Client {
    /// The field `padding` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const PADDING_VALUE: u32 = 0x00;

}

impl CMD_REALM_LIST_Client {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // padding: u32
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMD_REALM_LIST_Client {
    const OPCODE: u8 = 16;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // padding: u32
        let _padding = crate::util::read_u32_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(5);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // padding: u32
            let _padding = crate::util::tokio_read_u32_le(r).await?;
            // padding is expected to always be 0 (0)

            Ok(Self {
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(5);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // padding: u32
            let _padding = crate::util::astd_read_u32_le(r).await?;
            // padding is expected to always be 0 (0)

            Ok(Self {
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(5);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMD_REALM_LIST_Client;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    const RAW0: [u8; 5] = [ 0x10, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_realm/client.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Client0() {
        let expected = CMD_REALM_LIST_Client {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/client.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Client0() {
        let expected = CMD_REALM_LIST_Client {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/client.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Client0() {
        let expected = CMD_REALM_LIST_Client {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
