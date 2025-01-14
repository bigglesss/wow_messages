use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::LoginResult;
use crate::logon::version_8::SecurityFlag;
use crate::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm:224`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm#L224):
/// ```text
/// slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
///     u8 protocol_version = 0;
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[32] server_public_key;
///         u8 generator_length;
///         u8[generator_length] generator;
///         u8 large_safe_prime_length;
///         u8[large_safe_prime_length] large_safe_prime;
///         u8[32] salt;
///         u8[16] crc_salt;
///         SecurityFlag security_flag;
///         if (security_flag & PIN) {
///             u32 pin_grid_seed;
///             u8[16] pin_salt;
///         }
///         if (security_flag & UNKNOWN0) {
///             u8 unknown0;
///             u8 unknown1;
///             u8 unknown2;
///             u8 unknown3;
///             u64 unknown4;
///         }
///         if (security_flag & AUTHENTICATOR) {
///             u8 unknown5;
///         }
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_CHALLENGE_Server {
    pub result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    /// The field `protocol_version` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const PROTOCOL_VERSION_VALUE: u8 = 0x00;

}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&Self::PROTOCOL_VERSION_VALUE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                // server_public_key: u8[32]
                for i in server_public_key.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // generator_length: u8
                w.write_all(&(generator.len() as u8).to_le_bytes())?;

                // generator: u8[generator_length]
                for i in generator.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // large_safe_prime_length: u8
                w.write_all(&(large_safe_prime.len() as u8).to_le_bytes())?;

                // large_safe_prime: u8[large_safe_prime_length]
                for i in large_safe_prime.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // salt: u8[32]
                for i in salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // crc_salt: u8[16]
                for i in crc_salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // security_flag: SecurityFlag
                w.write_all(&(security_flag.as_int() as u8).to_le_bytes())?;

                if let Some(if_statement) = &security_flag.pin {
                    // pin_grid_seed: u32
                    w.write_all(&if_statement.pin_grid_seed.to_le_bytes())?;

                    // pin_salt: u8[16]
                    for i in if_statement.pin_salt.iter() {
                        w.write_all(&i.to_le_bytes())?;
                    }

                }

                if let Some(if_statement) = &security_flag.unknown0 {
                    // unknown0: u8
                    w.write_all(&if_statement.unknown0.to_le_bytes())?;

                    // unknown1: u8
                    w.write_all(&if_statement.unknown1.to_le_bytes())?;

                    // unknown2: u8
                    w.write_all(&if_statement.unknown2.to_le_bytes())?;

                    // unknown3: u8
                    w.write_all(&if_statement.unknown3.to_le_bytes())?;

                    // unknown4: u64
                    w.write_all(&if_statement.unknown4.to_le_bytes())?;

                }

                if let Some(if_statement) = &security_flag.authenticator {
                    // unknown5: u8
                    w.write_all(&if_statement.unknown5.to_le_bytes())?;

                }

            }
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0 => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1 => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol => {}
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced => {}
        }

        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0x00;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // protocol_version: u8
        let _protocol_version = crate::util::read_u8_le(r)?;
        // protocol_version is expected to always be 0 (0)

        // result: LoginResult
        let result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_public_key: u8[32]
                let mut server_public_key = [0_u8; 32];
                r.read_exact(&mut server_public_key)?;

                // generator_length: u8
                let generator_length = crate::util::read_u8_le(r)?;

                // generator: u8[generator_length]
                let mut generator = Vec::with_capacity(generator_length as usize);
                for i in 0..generator_length {
                    generator.push(crate::util::read_u8_le(r)?);
                }

                // large_safe_prime_length: u8
                let large_safe_prime_length = crate::util::read_u8_le(r)?;

                // large_safe_prime: u8[large_safe_prime_length]
                let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                for i in 0..large_safe_prime_length {
                    large_safe_prime.push(crate::util::read_u8_le(r)?);
                }

                // salt: u8[32]
                let mut salt = [0_u8; 32];
                r.read_exact(&mut salt)?;

                // crc_salt: u8[16]
                let mut crc_salt = [0_u8; 16];
                r.read_exact(&mut crc_salt)?;

                // security_flag: SecurityFlag
                let security_flag = SecurityFlag::new(crate::util::read_u8_le(r)?);

                let security_flag_PIN = if security_flag.is_PIN() {
                    // pin_grid_seed: u32
                    let pin_grid_seed = crate::util::read_u32_le(r)?;

                    // pin_salt: u8[16]
                    let mut pin_salt = [0_u8; 16];
                    r.read_exact(&mut pin_salt)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed,
                        pin_salt,
                    })
                }
                else {
                    None
                };

                let security_flag_UNKNOWN0 = if security_flag.is_UNKNOWN0() {
                    // unknown0: u8
                    let unknown0 = crate::util::read_u8_le(r)?;

                    // unknown1: u8
                    let unknown1 = crate::util::read_u8_le(r)?;

                    // unknown2: u8
                    let unknown2 = crate::util::read_u8_le(r)?;

                    // unknown3: u8
                    let unknown3 = crate::util::read_u8_le(r)?;

                    // unknown4: u64
                    let unknown4 = crate::util::read_u64_le(r)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0,
                        unknown1,
                        unknown2,
                        unknown3,
                        unknown4,
                    })
                }
                else {
                    None
                };

                let security_flag_AUTHENTICATOR = if security_flag.is_AUTHENTICATOR() {
                    // unknown5: u8
                    let unknown5 = crate::util::read_u8_le(r)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5,
                    })
                }
                else {
                    None
                };

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                    inner: security_flag.as_int(),
                    pin: security_flag_PIN,
                    unknown0: security_flag_UNKNOWN0,
                    authenticator: security_flag_AUTHENTICATOR,
                };

                CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
        };

        Ok(Self {
            result: result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
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
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // protocol_version: u8
            let _protocol_version = crate::util::tokio_read_u8_le(r).await?;
            // protocol_version is expected to always be 0 (0)

            // result: LoginResult
            let result: LoginResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_public_key: u8[32]
                    let mut server_public_key = [0_u8; 32];
                    r.read_exact(&mut server_public_key).await?;

                    // generator_length: u8
                    let generator_length = crate::util::tokio_read_u8_le(r).await?;

                    // generator: u8[generator_length]
                    let mut generator = Vec::with_capacity(generator_length as usize);
                    for i in 0..generator_length {
                        generator.push(crate::util::tokio_read_u8_le(r).await?);
                    }

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::tokio_read_u8_le(r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                    for i in 0..large_safe_prime_length {
                        large_safe_prime.push(crate::util::tokio_read_u8_le(r).await?);
                    }

                    // salt: u8[32]
                    let mut salt = [0_u8; 32];
                    r.read_exact(&mut salt).await?;

                    // crc_salt: u8[16]
                    let mut crc_salt = [0_u8; 16];
                    r.read_exact(&mut crc_salt).await?;

                    // security_flag: SecurityFlag
                    let security_flag = SecurityFlag::new(crate::util::tokio_read_u8_le(r).await?);

                    let security_flag_PIN = if security_flag.is_PIN() {
                        // pin_grid_seed: u32
                        let pin_grid_seed = crate::util::tokio_read_u32_le(r).await?;

                        // pin_salt: u8[16]
                        let mut pin_salt = [0_u8; 16];
                        r.read_exact(&mut pin_salt).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                            pin_grid_seed,
                            pin_salt,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_UNKNOWN0 = if security_flag.is_UNKNOWN0() {
                        // unknown0: u8
                        let unknown0 = crate::util::tokio_read_u8_le(r).await?;

                        // unknown1: u8
                        let unknown1 = crate::util::tokio_read_u8_le(r).await?;

                        // unknown2: u8
                        let unknown2 = crate::util::tokio_read_u8_le(r).await?;

                        // unknown3: u8
                        let unknown3 = crate::util::tokio_read_u8_le(r).await?;

                        // unknown4: u64
                        let unknown4 = crate::util::tokio_read_u64_le(r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                            unknown0,
                            unknown1,
                            unknown2,
                            unknown3,
                            unknown4,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_AUTHENTICATOR = if security_flag.is_AUTHENTICATOR() {
                        // unknown5: u8
                        let unknown5 = crate::util::tokio_read_u8_le(r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                            unknown5,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                        inner: security_flag.as_int(),
                        pin: security_flag_PIN,
                        unknown0: security_flag_UNKNOWN0,
                        authenticator: security_flag_AUTHENTICATOR,
                    };

                    CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag,
                        server_public_key,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
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
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // protocol_version: u8
            let _protocol_version = crate::util::astd_read_u8_le(r).await?;
            // protocol_version is expected to always be 0 (0)

            // result: LoginResult
            let result: LoginResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_public_key: u8[32]
                    let mut server_public_key = [0_u8; 32];
                    r.read_exact(&mut server_public_key).await?;

                    // generator_length: u8
                    let generator_length = crate::util::astd_read_u8_le(r).await?;

                    // generator: u8[generator_length]
                    let mut generator = Vec::with_capacity(generator_length as usize);
                    for i in 0..generator_length {
                        generator.push(crate::util::astd_read_u8_le(r).await?);
                    }

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::astd_read_u8_le(r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                    for i in 0..large_safe_prime_length {
                        large_safe_prime.push(crate::util::astd_read_u8_le(r).await?);
                    }

                    // salt: u8[32]
                    let mut salt = [0_u8; 32];
                    r.read_exact(&mut salt).await?;

                    // crc_salt: u8[16]
                    let mut crc_salt = [0_u8; 16];
                    r.read_exact(&mut crc_salt).await?;

                    // security_flag: SecurityFlag
                    let security_flag = SecurityFlag::new(crate::util::astd_read_u8_le(r).await?);

                    let security_flag_PIN = if security_flag.is_PIN() {
                        // pin_grid_seed: u32
                        let pin_grid_seed = crate::util::astd_read_u32_le(r).await?;

                        // pin_salt: u8[16]
                        let mut pin_salt = [0_u8; 16];
                        r.read_exact(&mut pin_salt).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                            pin_grid_seed,
                            pin_salt,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_UNKNOWN0 = if security_flag.is_UNKNOWN0() {
                        // unknown0: u8
                        let unknown0 = crate::util::astd_read_u8_le(r).await?;

                        // unknown1: u8
                        let unknown1 = crate::util::astd_read_u8_le(r).await?;

                        // unknown2: u8
                        let unknown2 = crate::util::astd_read_u8_le(r).await?;

                        // unknown3: u8
                        let unknown3 = crate::util::astd_read_u8_le(r).await?;

                        // unknown4: u64
                        let unknown4 = crate::util::astd_read_u64_le(r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                            unknown0,
                            unknown1,
                            unknown2,
                            unknown3,
                            unknown4,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_AUTHENTICATOR = if security_flag.is_AUTHENTICATOR() {
                        // unknown5: u8
                        let unknown5 = crate::util::astd_read_u8_le(r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                            unknown5,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                        inner: security_flag.as_int(),
                        pin: security_flag_PIN,
                        unknown0: security_flag_UNKNOWN0,
                        authenticator: security_flag_AUTHENTICATOR,
                    };

                    CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag,
                        server_public_key,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn size(&self) -> usize {
        1 // protocol_version: u8
        + self.result.size() // result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin>,
    unknown0: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0>,
    authenticator: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator>,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    pub const fn empty() -> Self {
        Self {
            inner: 0,
            pin: None,
            unknown0: None,
            authenticator: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.pin.is_none()
        && self.unknown0.is_none()
        && self.authenticator.is_none()
    }

    pub const fn new_PIN(pin: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            unknown0: None,
            authenticator: None,
        }
    }

    pub fn set_PIN(mut self, pin: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin) -> Self {
        self.inner |= SecurityFlag::PIN;
        self.pin = Some(pin);
        self
    }

    pub const fn get_PIN(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin> {
        self.pin.as_ref()
    }

    pub fn clear_PIN(mut self) -> Self {
        self.inner &= SecurityFlag::PIN.reverse_bits();
        self.pin = None;
        self
    }

    pub const fn new_UNKNOWN0(unknown0: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0) -> Self {
        Self {
            inner: SecurityFlag::UNKNOWN0,
            pin: None,
            unknown0: Some(unknown0),
            authenticator: None,
        }
    }

    pub fn set_UNKNOWN0(mut self, unknown0: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0) -> Self {
        self.inner |= SecurityFlag::UNKNOWN0;
        self.unknown0 = Some(unknown0);
        self
    }

    pub const fn get_UNKNOWN0(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0> {
        self.unknown0.as_ref()
    }

    pub fn clear_UNKNOWN0(mut self) -> Self {
        self.inner &= SecurityFlag::UNKNOWN0.reverse_bits();
        self.unknown0 = None;
        self
    }

    pub const fn new_AUTHENTICATOR(authenticator: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            unknown0: None,
            authenticator: Some(authenticator),
        }
    }

    pub fn set_AUTHENTICATOR(mut self, authenticator: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator) -> Self {
        self.inner |= SecurityFlag::AUTHENTICATOR;
        self.authenticator = Some(authenticator);
        self
    }

    pub const fn get_AUTHENTICATOR(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator> {
        self.authenticator.as_ref()
    }

    pub fn clear_AUTHENTICATOR(mut self) -> Self {
        self.inner &= SecurityFlag::AUTHENTICATOR.reverse_bits();
        self.authenticator = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    pub(crate) fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.pin {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.unknown0 {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.authenticator {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
    pub pin_grid_seed: u32,
    pub pin_salt: [u8; 16],
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
    pub(crate) fn size(&self) -> usize {
        4 // pin_grid_seed: u32
        + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
    pub unknown0: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u64,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
    pub(crate) fn size(&self) -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
    pub unknown5: u8,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
    pub(crate) fn size(&self) -> usize {
        1 // unknown5: u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    Success {
        crc_salt: [u8; 16],
        generator: Vec<u8>,
        large_safe_prime: Vec<u8>,
        salt: [u8; 32],
        security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag,
        server_public_key: [u8; 32],
    },
    FailUnknown0,
    FailUnknown1,
    FailBanned,
    FailUnknownAccount,
    FailIncorrectPassword,
    FailAlreadyOnline,
    FailNoTime,
    FailDbBusy,
    FailVersionInvalid,
    LoginDownloadFile,
    FailInvalidServer,
    FailSuspended,
    FailNoAccess,
    SuccessSurvey,
    FailParentalcontrol,
    FailLockedEnforced,
}

impl Default for CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Success {
            crc_salt: Default::default(),
            generator: Default::default(),
            large_safe_prime: Default::default(),
            salt: Default::default(),
            security_flag: Default::default(),
            server_public_key: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::FailUnknown0 => 1,
            Self::FailUnknown1 => 2,
            Self::FailBanned => 3,
            Self::FailUnknownAccount => 4,
            Self::FailIncorrectPassword => 5,
            Self::FailAlreadyOnline => 6,
            Self::FailNoTime => 7,
            Self::FailDbBusy => 8,
            Self::FailVersionInvalid => 9,
            Self::LoginDownloadFile => 10,
            Self::FailInvalidServer => 11,
            Self::FailSuspended => 12,
            Self::FailNoAccess => 13,
            Self::SuccessSurvey => 14,
            Self::FailParentalcontrol => 15,
            Self::FailLockedEnforced => 16,
        }
    }

}

impl CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                1
                + 16 * core::mem::size_of::<u8>() // crc_salt: u8[16]
                + generator.len() * core::mem::size_of::<u8>() // generator: u8[generator_length]
                + 1 // generator_length: u8
                + large_safe_prime.len() * core::mem::size_of::<u8>() // large_safe_prime: u8[large_safe_prime_length]
                + 1 // large_safe_prime_length: u8
                + 32 * core::mem::size_of::<u8>() // salt: u8[32]
                + security_flag.size() // security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag
                + 32 * core::mem::size_of::<u8>() // server_public_key: u8[32]
            }
            Self::FailUnknown0 => {
                1
            }
            Self::FailUnknown1 => {
                1
            }
            Self::FailBanned => {
                1
            }
            Self::FailUnknownAccount => {
                1
            }
            Self::FailIncorrectPassword => {
                1
            }
            Self::FailAlreadyOnline => {
                1
            }
            Self::FailNoTime => {
                1
            }
            Self::FailDbBusy => {
                1
            }
            Self::FailVersionInvalid => {
                1
            }
            Self::LoginDownloadFile => {
                1
            }
            Self::FailInvalidServer => {
                1
            }
            Self::FailSuspended => {
                1
            }
            Self::FailNoAccess => {
                1
            }
            Self::SuccessSurvey => {
                1
            }
            Self::FailParentalcontrol => {
                1
            }
            Self::FailLockedEnforced => {
                1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_CHALLENGE_Server;
    use crate::logon::version_8::LoginResult;
    use crate::logon::version_8::SecurityFlag;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 119] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 264.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 264.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 264.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 139] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 317.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_PIN(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed: 0xDEADBEEF,
                        pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 317.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_PIN(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed: 0xDEADBEEF,
                        pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 317.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_PIN(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed: 0xDEADBEEF,
                        pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 120] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x04, 0x01, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 377.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server2() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 377.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server2() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 377.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server2() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 131] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x02, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 432.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server3() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 432.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server3() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 432.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server3() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    const RAW4: [u8; 3] = [ 0x00, 0x00, 0x05, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 496.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server4() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW4)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 496.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server4() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 496.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server4() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    const RAW5: [u8; 132] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x06, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x01, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 506.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server5() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW5)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 506.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server5() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 506.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server5() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_UNKNOWN0(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Unknown0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    })
                    .set_AUTHENTICATOR(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        unknown5: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    const RAW6: [u8; 3] = [ 0x00, 0x00, 0x05, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 573.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server6() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW6)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 573.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server6() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 573.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server6() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

}
