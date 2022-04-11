use std::convert::{TryFrom, TryInto};
use crate::logon::all::Locale;
use crate::logon::all::Os;
use crate::logon::all::Platform;
use crate::logon::all::Version;
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm#L40):
/// ```text
/// clogin CMD_AUTH_LOGON_CHALLENGE_Client = 0x0 {
///     u8 protocol_version;
///     u16 size = self.size;
///     u32 game_name = "\0WoW";
///     Version version;
///     Platform platform;
///     Os os;
///     Locale locale;
///     u32 utc_timezone_offset;
///     u32_be client_ip_address;
///     u8 account_name_length;
///     String[account_name_length] account_name;
/// }
/// ```
pub struct CMD_AUTH_LOGON_CHALLENGE_Client {
    pub protocol_version: u8,
    pub version: Version,
    pub platform: Platform,
    pub os: Os,
    pub locale: Locale,
    pub utc_timezone_offset: u32,
    pub client_ip_address: u32,
    pub account_name: String,
}

impl ClientMessage for CMD_AUTH_LOGON_CHALLENGE_Client {
    const OPCODE: u8 = 0x00;
}
impl CMD_AUTH_LOGON_CHALLENGE_Client {
    /// The field `game_name` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `5730135` |
    /// | Hex | `0x576f57` |
    /// | Original | `"\0WoW"` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const GAME_NAME_VALUE: u32 = 0x576f57;

}

impl ReadableAndWritable for CMD_AUTH_LOGON_CHALLENGE_Client {
    type Error = CMD_AUTH_LOGON_CHALLENGE_ClientError;

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

}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_Client {
    fn size(&self) -> usize {
        1 // protocol_version: u8
        + 2 // size: u16
        + 4 // game_name: u32
        + Version::size() // version: Version
        + Platform::size() // platform: Platform
        + Os::size() // os: Os
        + Locale::size() // locale: Locale
        + 4 // utc_timezone_offset: u32
        + 4 // client_ip_address: u32_be
        + 1 // account_name_length: u8
        + self.account_name.len() // account_name: String[account_name_length]
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_Client {
    fn maximum_possible_size() -> usize {
        1 // protocol_version: u8
        + 2 // size: u16
        + 4 // game_name: u32
        + Version::maximum_possible_size() // version: Version
        + Platform::maximum_possible_size() // platform: Platform
        + Os::maximum_possible_size() // os: Os
        + Locale::maximum_possible_size() // locale: Locale
        + 4 // utc_timezone_offset: u32
        + 4 // client_ip_address: u32_be
        + 1 // account_name_length: u8
        + 255 // account_name: String[account_name_length]
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_LOGON_CHALLENGE_ClientError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMD_AUTH_LOGON_CHALLENGE_ClientError {}
impl std::fmt::Display for CMD_AUTH_LOGON_CHALLENGE_ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_LOGON_CHALLENGE_ClientError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMD_AUTH_LOGON_CHALLENGE_ClientError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_LOGON_CHALLENGE_Client;
    use crate::VariableSized;
    use crate::logon::all::Locale;
    use crate::logon::all::Os;
    use crate::logon::all::Platform;
    use crate::logon::all::Version;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm` line 56.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Client0() {
        let raw: Vec<u8> = vec![ 0x00, 0x03, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00,
             0x01, 0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57,
             0x00, 0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
             0x01, 0x01, 0x41, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Client {
            protocol_version: 0x3,
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
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
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
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
