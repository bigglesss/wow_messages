use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::{Realm, RealmError};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_REALM_LIST_Server {
    pub realms: Vec<Realm>,
}

impl ServerMessage for CMD_REALM_LIST_Server {
    const OPCODE: u8 = 0x10;
}
impl CMD_REALM_LIST_Server {
    pub const HEADER_PADDING_VALUE: u32 = 0x00;

    pub const FOOTER_PADDING_VALUE: u16 = 0x00;

}

impl ReadableAndWritable for CMD_REALM_LIST_Server {
    type Error = CMD_REALM_LIST_ServerError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // size: u16
        let _size = crate::util::read_u16_le(r)?;
        // size is expected to always be self.size (0)

        // header_padding: u32
        let _header_padding = crate::util::read_u32_le(r)?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u8
        let number_of_realms = crate::util::read_u8_le(r)?;

        // realms: Realm[number_of_realms]
        let mut realms = Vec::with_capacity(number_of_realms as usize);
        for i in 0..number_of_realms {
            realms.push(Realm::read(r)?);
        }

        // footer_padding: u16
        let _footer_padding = crate::util::read_u16_le(r)?;
        // footer_padding is expected to always be 0 (0)

        Ok(Self {
            realms,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // size: u16
        w.write_all(&((self.size() - 2) as u16).to_le_bytes())?;

        // header_padding: u32
        w.write_all(&Self::HEADER_PADDING_VALUE.to_le_bytes())?;

        // number_of_realms: u8
        w.write_all(&(self.realms.len() as u8).to_le_bytes())?;

        // realms: Realm[number_of_realms]
        for i in self.realms.iter() {
            i.write(w)?;
        }

        // footer_padding: u16
        w.write_all(&Self::FOOTER_PADDING_VALUE.to_le_bytes())?;

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
            // size: u16
            let _size = crate::util::tokio_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // header_padding: u32
            let _header_padding = crate::util::tokio_read_u32_le(r).await?;
            // header_padding is expected to always be 0 (0)

            // number_of_realms: u8
            let number_of_realms = crate::util::tokio_read_u8_le(r).await?;

            // realms: Realm[number_of_realms]
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for i in 0..number_of_realms {
                realms.push(Realm::tokio_read(r).await?);
            }

            // footer_padding: u16
            let _footer_padding = crate::util::tokio_read_u16_le(r).await?;
            // footer_padding is expected to always be 0 (0)

            Ok(Self {
                realms,
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // size: u16
            w.write_all(&((self.size() - 2) as u16).to_le_bytes()).await?;

            // header_padding: u32
            w.write_all(&Self::HEADER_PADDING_VALUE.to_le_bytes()).await?;

            // number_of_realms: u8
            w.write_all(&(self.realms.len() as u8).to_le_bytes()).await?;

            // realms: Realm[number_of_realms]
            for i in self.realms.iter() {
                i.tokio_write(w).await?;
            }

            // footer_padding: u16
            w.write_all(&Self::FOOTER_PADDING_VALUE.to_le_bytes()).await?;

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
            // size: u16
            let _size = crate::util::astd_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // header_padding: u32
            let _header_padding = crate::util::astd_read_u32_le(r).await?;
            // header_padding is expected to always be 0 (0)

            // number_of_realms: u8
            let number_of_realms = crate::util::astd_read_u8_le(r).await?;

            // realms: Realm[number_of_realms]
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for i in 0..number_of_realms {
                realms.push(Realm::astd_read(r).await?);
            }

            // footer_padding: u16
            let _footer_padding = crate::util::astd_read_u16_le(r).await?;
            // footer_padding is expected to always be 0 (0)

            Ok(Self {
                realms,
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // size: u16
            w.write_all(&((self.size() - 2) as u16).to_le_bytes()).await?;

            // header_padding: u32
            w.write_all(&Self::HEADER_PADDING_VALUE.to_le_bytes()).await?;

            // number_of_realms: u8
            w.write_all(&(self.realms.len() as u8).to_le_bytes()).await?;

            // realms: Realm[number_of_realms]
            for i in self.realms.iter() {
                i.astd_write(w).await?;
            }

            // footer_padding: u16
            w.write_all(&Self::FOOTER_PADDING_VALUE.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for CMD_REALM_LIST_Server {
    fn size(&self) -> usize {
        0
        + 2 // size: u16
        + 4 // header_padding: u32
        + 1 // number_of_realms: u8
        + self.realms.iter().fold(0, |acc, x| acc + x.size()) // realms: Realm[number_of_realms]
        + 2 // footer_padding: u16
    }
}

impl MaximumPossibleSized for CMD_REALM_LIST_Server {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum CMD_REALM_LIST_ServerError {
    Io(std::io::Error),
    Realm(RealmError),
}

impl std::error::Error for CMD_REALM_LIST_ServerError {}
impl std::fmt::Display for CMD_REALM_LIST_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Realm(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_REALM_LIST_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RealmError> for CMD_REALM_LIST_ServerError {
    fn from(e: RealmError) -> Self {
        Self::Realm(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMD_REALM_LIST_Server;
    use crate::VariableSized;
    use crate::logon::version_2::Population;
    use crate::logon::version_2::Realm;
    use crate::logon::version_2::RealmCategory;
    use crate::logon::version_2::RealmFlag;
    use crate::logon::version_2::RealmType;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server0() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_NONE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server0() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_NONE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server0() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_NONE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server1() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x03, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_INVALID().set_OFFLINE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server1() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x03, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_INVALID().set_OFFLINE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server1() {
        let raw: Vec<u8> = vec![ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x03, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8,
             0x43, 0x01, 0x00, 0x02, 0x00, 0x00, ];

        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                    flag: RealmFlag::empty()
                        .set_INVALID().set_OFFLINE(),
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RED_FULL,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::DEFAULT,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
