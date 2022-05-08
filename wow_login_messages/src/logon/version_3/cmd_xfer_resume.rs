use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_XFER_RESUME {
    pub offset: u64,
}

impl ClientMessage for CMD_XFER_RESUME {
    const OPCODE: u8 = 0x33;
}
impl ReadableAndWritable for CMD_XFER_RESUME {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // offset: u64
        let offset = crate::util::read_u64_le(r)?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // offset: u64
            let offset = crate::util::tokio_read_u64_le(r).await?;

            Ok(Self {
                offset,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // offset: u64
            w.write_all(&self.offset.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // offset: u64
            let offset = crate::util::astd_read_u64_le(r).await?;

            Ok(Self {
                offset,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // offset: u64
            w.write_all(&self.offset.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMD_XFER_RESUME {}

impl MaximumPossibleSized for CMD_XFER_RESUME {
    fn maximum_possible_size() -> usize {
        0
        + 8 // offset: u64
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMD_XFER_RESUME;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_RESUME0() {
        let raw: Vec<u8> = vec![ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(CMD_XFER_RESUME::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_CMD_XFER_RESUME0() {
        let raw: Vec<u8> = vec![ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(CMD_XFER_RESUME::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_CMD_XFER_RESUME0() {
        let raw: Vec<u8> = vec![ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(CMD_XFER_RESUME::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
