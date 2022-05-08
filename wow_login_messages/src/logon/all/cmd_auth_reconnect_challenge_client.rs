use std::convert::{TryFrom, TryInto};
use crate::logon::all::Locale;
use crate::logon::all::Os;
use crate::logon::all::Platform;
use crate::logon::all::Version;
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_RECONNECT_CHALLENGE_Client {
    pub protocol_version: u8,
    pub version: Version,
    pub platform: Platform,
    pub os: Os,
    pub locale: Locale,
    pub utc_timezone_offset: u32,
    pub client_ip_address: u32,
    pub account_name: String,
}

impl ClientMessage for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    const OPCODE: u8 = 0x02;
}
impl CMD_AUTH_RECONNECT_CHALLENGE_Client {
    pub const GAME_NAME_VALUE: u32 = 0x576f57;

}

impl ReadableAndWritable for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    type Error = CMD_AUTH_RECONNECT_CHALLENGE_ClientError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // protocol_version: u8
        let protocol_version = crate::util::read_u8_le(r)?;

        // size: u16
        let _size = crate::util::read_u16_le(r)?;
        // size is expected to always be self.size (0)

        // game_name: u32
        let _game_name = crate::util::read_u32_le(r)?;
        // game_name is expected to always be "\0WoW" (5730135)

        // version: Version
        let version = Version::read(r)?;

        // platform: Platform
        let platform = Platform::read(r)?;

        // os: Os
        let os = Os::read(r)?;

        // locale: Locale
        let locale = Locale::read(r)?;

        // utc_timezone_offset: u32
        let utc_timezone_offset = crate::util::read_u32_le(r)?;

        // client_ip_address: u32_be
        let client_ip_address = crate::util::read_u32_be(r)?;

        // account_name_length: u8
        let account_name_length = crate::util::read_u8_le(r)?;

        // account_name: String[account_name_length]
        let account_name = crate::util::read_fixed_string_to_vec(r, account_name_length as usize)?;
        let account_name = String::from_utf8(account_name)?;

        Ok(Self {
            protocol_version,
            version,
            platform,
            os,
            locale,
            utc_timezone_offset,
            client_ip_address,
            account_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&self.protocol_version.to_le_bytes())?;

        // size: u16
        w.write_all(&((self.size() - 3) as u16).to_le_bytes())?;

        // game_name: u32
        w.write_all(&Self::GAME_NAME_VALUE.to_le_bytes())?;

        // version: Version
        self.version.write(w)?;

        // platform: Platform
        self.platform.write(w)?;

        // os: Os
        self.os.write(w)?;

        // locale: Locale
        self.locale.write(w)?;

        // utc_timezone_offset: u32
        w.write_all(&self.utc_timezone_offset.to_le_bytes())?;

        // client_ip_address: u32_be
        w.write_all(&self.client_ip_address.to_be_bytes())?;

        // account_name_length: u8
        w.write_all(&(self.account_name.len() as u8).to_le_bytes())?;

        // account_name: String[account_name_length]
        w.write_all(self.account_name.as_bytes())?;

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
            // protocol_version: u8
            let protocol_version = crate::util::tokio_read_u8_le(r).await?;

            // size: u16
            let _size = crate::util::tokio_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // game_name: u32
            let _game_name = crate::util::tokio_read_u32_le(r).await?;
            // game_name is expected to always be "\0WoW" (5730135)

            // version: Version
            let version = Version::tokio_read(r).await?;

            // platform: Platform
            let platform = Platform::tokio_read(r).await?;

            // os: Os
            let os = Os::tokio_read(r).await?;

            // locale: Locale
            let locale = Locale::tokio_read(r).await?;

            // utc_timezone_offset: u32
            let utc_timezone_offset = crate::util::tokio_read_u32_le(r).await?;

            // client_ip_address: u32_be
            let client_ip_address = crate::util::tokio_read_u32_be(r).await?;

            // account_name_length: u8
            let account_name_length = crate::util::tokio_read_u8_le(r).await?;

            // account_name: String[account_name_length]
            let account_name = crate::util::tokio_read_fixed_string_to_vec(r, account_name_length as usize).await?;
            let account_name = String::from_utf8(account_name)?;

            Ok(Self {
                protocol_version,
                version,
                platform,
                os,
                locale,
                utc_timezone_offset,
                client_ip_address,
                account_name,
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

            // protocol_version: u8
            w.write_all(&self.protocol_version.to_le_bytes()).await?;

            // size: u16
            w.write_all(&((self.size() - 3) as u16).to_le_bytes()).await?;

            // game_name: u32
            w.write_all(&Self::GAME_NAME_VALUE.to_le_bytes()).await?;

            // version: Version
            self.version.tokio_write(w).await?;

            // platform: Platform
            self.platform.tokio_write(w).await?;

            // os: Os
            self.os.tokio_write(w).await?;

            // locale: Locale
            self.locale.tokio_write(w).await?;

            // utc_timezone_offset: u32
            w.write_all(&self.utc_timezone_offset.to_le_bytes()).await?;

            // client_ip_address: u32_be
            w.write_all(&self.client_ip_address.to_be_bytes()).await?;

            // account_name_length: u8
            w.write_all(&(self.account_name.len() as u8).to_le_bytes()).await?;

            // account_name: String[account_name_length]
            w.write_all(self.account_name.as_bytes()).await?;

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
            // protocol_version: u8
            let protocol_version = crate::util::astd_read_u8_le(r).await?;

            // size: u16
            let _size = crate::util::astd_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // game_name: u32
            let _game_name = crate::util::astd_read_u32_le(r).await?;
            // game_name is expected to always be "\0WoW" (5730135)

            // version: Version
            let version = Version::astd_read(r).await?;

            // platform: Platform
            let platform = Platform::astd_read(r).await?;

            // os: Os
            let os = Os::astd_read(r).await?;

            // locale: Locale
            let locale = Locale::astd_read(r).await?;

            // utc_timezone_offset: u32
            let utc_timezone_offset = crate::util::astd_read_u32_le(r).await?;

            // client_ip_address: u32_be
            let client_ip_address = crate::util::astd_read_u32_be(r).await?;

            // account_name_length: u8
            let account_name_length = crate::util::astd_read_u8_le(r).await?;

            // account_name: String[account_name_length]
            let account_name = crate::util::astd_read_fixed_string_to_vec(r, account_name_length as usize).await?;
            let account_name = String::from_utf8(account_name)?;

            Ok(Self {
                protocol_version,
                version,
                platform,
                os,
                locale,
                utc_timezone_offset,
                client_ip_address,
                account_name,
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

            // protocol_version: u8
            w.write_all(&self.protocol_version.to_le_bytes()).await?;

            // size: u16
            w.write_all(&((self.size() - 3) as u16).to_le_bytes()).await?;

            // game_name: u32
            w.write_all(&Self::GAME_NAME_VALUE.to_le_bytes()).await?;

            // version: Version
            self.version.astd_write(w).await?;

            // platform: Platform
            self.platform.astd_write(w).await?;

            // os: Os
            self.os.astd_write(w).await?;

            // locale: Locale
            self.locale.astd_write(w).await?;

            // utc_timezone_offset: u32
            w.write_all(&self.utc_timezone_offset.to_le_bytes()).await?;

            // client_ip_address: u32_be
            w.write_all(&self.client_ip_address.to_be_bytes()).await?;

            // account_name_length: u8
            w.write_all(&(self.account_name.len() as u8).to_le_bytes()).await?;

            // account_name: String[account_name_length]
            w.write_all(self.account_name.as_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    fn size(&self) -> usize {
        0
        + 1 // protocol_version: u8
        + 2 // size: u16
        + 4 // game_name: u32
        + Version::size() // version: Version
        + 4 // platform: Platform
        + 4 // os: Os
        + 4 // locale: Locale
        + 4 // utc_timezone_offset: u32
        + 4 // client_ip_address: u32_be
        + 1 // account_name_length: u8
        + self.account_name.len() // account_name: String
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    fn maximum_possible_size() -> usize {
        0
        + 1 // protocol_version: u8
        + 2 // size: u16
        + 4 // game_name: u32
        + 5 // version: Version
        + 4 // platform: Platform
        + 4 // os: Os
        + 4 // locale: Locale
        + 4 // utc_timezone_offset: u32
        + 4 // client_ip_address: u32_be
        + 1 // account_name_length: u8
        + 256 // account_name: String
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_ClientError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMD_AUTH_RECONNECT_CHALLENGE_ClientError {}
impl std::fmt::Display for CMD_AUTH_RECONNECT_CHALLENGE_ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_RECONNECT_CHALLENGE_ClientError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMD_AUTH_RECONNECT_CHALLENGE_ClientError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMD_AUTH_RECONNECT_CHALLENGE_Client;
    use crate::VariableSized;
    use crate::logon::all::Locale;
    use crate::logon::all::Os;
    use crate::logon::all::Platform;
    use crate::logon::all::Version;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Client0() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x01, 0x41, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_CHALLENGE_Client0() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x01, 0x41, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_CHALLENGE_Client0() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x01, 0x41, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Client1() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x2E, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x10, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
             0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("ABCDEFGHIJKLMNOP"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_CHALLENGE_Client1() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x2E, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x10, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
             0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("ABCDEFGHIJKLMNOP"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_CHALLENGE_Client1() {
        let raw: Vec<u8> = vec![ 0x02, 0x02, 0x2E, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x10, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A,
             0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: 0x2,
            version: Version {
                major: 1,
                minor: 12,
                patch: 1,
                build: 5875,
            },
            platform: Platform::X86,
            os: Os::WINDOWS,
            locale: Locale::EN_GB,
            utc_timezone_offset: 0x3C,
            client_ip_address: 0x7F000001,
            account_name: String::from("ABCDEFGHIJKLMNOP"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
