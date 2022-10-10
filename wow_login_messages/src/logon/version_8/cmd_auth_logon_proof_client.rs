use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::TelemetryKey;
use crate::logon::version_8::SecurityFlag;
use crate::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Reply after successful [`CMD_AUTH_LOGON_CHALLENGE_Server`](crate::logon::version_8::CMD_AUTH_LOGON_CHALLENGE_Server).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm:317`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm#L317):
/// ```text
/// clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
///     u8[32] client_public_key;
///     u8[20] client_proof;
///     u8[20] crc_hash;
///     u8 number_of_telemetry_keys;
///     TelemetryKey[number_of_telemetry_keys] telemetry_keys;
///     SecurityFlag security_flag;
///     if (security_flag & PIN) {
///         u8[16] pin_salt;
///         u8[20] pin_hash;
///     }
///     if (security_flag & UNKNOWN0) {
///         u8 unknown0;
///         u8 unknown1;
///         u8 unknown2;
///         u8 unknown3;
///         u64 unknown4;
///     }
///     if (security_flag & AUTHENTICATOR) {
///         u8 unknown5;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Client {
    pub client_public_key: [u8; 32],
    pub client_proof: [u8; 20],
    pub crc_hash: [u8; 20],
    pub telemetry_keys: Vec<TelemetryKey>,
    pub security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag,
}

impl CMD_AUTH_LOGON_PROOF_Client {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // client_public_key: u8[32]
        for i in self.client_public_key.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // crc_hash: u8[20]
        for i in self.crc_hash.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // number_of_telemetry_keys: u8
        w.write_all(&(self.telemetry_keys.len() as u8).to_le_bytes())?;

        // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        for i in self.telemetry_keys.iter() {
            i.write_into_vec(w)?;
        }

        // security_flag: SecurityFlag
        w.write_all(&(self.security_flag.as_int() as u8).to_le_bytes())?;

        if let Some(if_statement) = &self.security_flag.pin {
            // pin_salt: u8[16]
            for i in if_statement.pin_salt.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // pin_hash: u8[20]
            for i in if_statement.pin_hash.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        if let Some(if_statement) = &self.security_flag.unknown0 {
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

        if let Some(if_statement) = &self.security_flag.authenticator {
            // unknown5: u8
            w.write_all(&if_statement.unknown5.to_le_bytes())?;

        }

        Ok(())
    }
}

impl ClientMessage for CMD_AUTH_LOGON_PROOF_Client {
    const OPCODE: u8 = 0x01;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // client_public_key: u8[32]
        let mut client_public_key = [0_u8; 32];
        r.read_exact(&mut client_public_key)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // crc_hash: u8[20]
        let mut crc_hash = [0_u8; 20];
        r.read_exact(&mut crc_hash)?;

        // number_of_telemetry_keys: u8
        let number_of_telemetry_keys = crate::util::read_u8_le(r)?;

        // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
        for i in 0..number_of_telemetry_keys {
            telemetry_keys.push(TelemetryKey::read(r)?);
        }

        // security_flag: SecurityFlag
        let security_flag = SecurityFlag::new(crate::util::read_u8_le(r)?);

        let security_flag_PIN = if security_flag.is_PIN() {
            // pin_salt: u8[16]
            let mut pin_salt = [0_u8; 16];
            r.read_exact(&mut pin_salt)?;

            // pin_hash: u8[20]
            let mut pin_hash = [0_u8; 20];
            r.read_exact(&mut pin_hash)?;

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                pin_hash,
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

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
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

            Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                unknown5,
            })
        }
        else {
            None
        };

        let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
            inner: security_flag.as_int(),
            pin: security_flag_PIN,
            unknown0: security_flag_UNKNOWN0,
            authenticator: security_flag_AUTHENTICATOR,
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag,
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
            // client_public_key: u8[32]
            let mut client_public_key = [0_u8; 32];
            r.read_exact(&mut client_public_key).await?;

            // client_proof: u8[20]
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof).await?;

            // crc_hash: u8[20]
            let mut crc_hash = [0_u8; 20];
            r.read_exact(&mut crc_hash).await?;

            // number_of_telemetry_keys: u8
            let number_of_telemetry_keys = crate::util::tokio_read_u8_le(r).await?;

            // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
            let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
            for i in 0..number_of_telemetry_keys {
                telemetry_keys.push(TelemetryKey::tokio_read(r).await?);
            }

            // security_flag: SecurityFlag
            let security_flag = SecurityFlag::new(crate::util::tokio_read_u8_le(r).await?);

            let security_flag_PIN = if security_flag.is_PIN() {
                // pin_salt: u8[16]
                let mut pin_salt = [0_u8; 16];
                r.read_exact(&mut pin_salt).await?;

                // pin_hash: u8[20]
                let mut pin_hash = [0_u8; 20];
                r.read_exact(&mut pin_hash).await?;

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash,
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

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
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

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    unknown5,
                })
            }
            else {
                None
            };

            let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
                inner: security_flag.as_int(),
                pin: security_flag_PIN,
                unknown0: security_flag_UNKNOWN0,
                authenticator: security_flag_AUTHENTICATOR,
            };

            Ok(Self {
                client_public_key,
                client_proof,
                crc_hash,
                telemetry_keys,
                security_flag,
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
            // client_public_key: u8[32]
            let mut client_public_key = [0_u8; 32];
            r.read_exact(&mut client_public_key).await?;

            // client_proof: u8[20]
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof).await?;

            // crc_hash: u8[20]
            let mut crc_hash = [0_u8; 20];
            r.read_exact(&mut crc_hash).await?;

            // number_of_telemetry_keys: u8
            let number_of_telemetry_keys = crate::util::astd_read_u8_le(r).await?;

            // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
            let mut telemetry_keys = Vec::with_capacity(number_of_telemetry_keys as usize);
            for i in 0..number_of_telemetry_keys {
                telemetry_keys.push(TelemetryKey::astd_read(r).await?);
            }

            // security_flag: SecurityFlag
            let security_flag = SecurityFlag::new(crate::util::astd_read_u8_le(r).await?);

            let security_flag_PIN = if security_flag.is_PIN() {
                // pin_salt: u8[16]
                let mut pin_salt = [0_u8; 16];
                r.read_exact(&mut pin_salt).await?;

                // pin_hash: u8[20]
                let mut pin_hash = [0_u8; 20];
                r.read_exact(&mut pin_hash).await?;

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash,
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

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
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

                Some(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    unknown5,
                })
            }
            else {
                None
            };

            let security_flag = CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
                inner: security_flag.as_int(),
                pin: security_flag_PIN,
                unknown0: security_flag_UNKNOWN0,
                authenticator: security_flag_AUTHENTICATOR,
            };

            Ok(Self {
                client_public_key,
                client_proof,
                crc_hash,
                telemetry_keys,
                security_flag,
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

impl CMD_AUTH_LOGON_PROOF_Client {
    pub(crate) fn size(&self) -> usize {
        32 * core::mem::size_of::<u8>() // client_public_key: u8[32]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + self.telemetry_keys.len() * 30 // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + self.security_flag.size() // security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin>,
    unknown0: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0>,
    authenticator: Option<CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator>,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
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

    pub const fn new_PIN(pin: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            unknown0: None,
            authenticator: None,
        }
    }

    pub fn set_PIN(mut self, pin: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin) -> Self {
        self.inner |= SecurityFlag::PIN;
        self.pin = Some(pin);
        self
    }

    pub const fn get_PIN(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin> {
        self.pin.as_ref()
    }

    pub fn clear_PIN(mut self) -> Self {
        self.inner &= SecurityFlag::PIN.reverse_bits();
        self.pin = None;
        self
    }

    pub const fn new_UNKNOWN0(unknown0: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0) -> Self {
        Self {
            inner: SecurityFlag::UNKNOWN0,
            pin: None,
            unknown0: Some(unknown0),
            authenticator: None,
        }
    }

    pub fn set_UNKNOWN0(mut self, unknown0: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0) -> Self {
        self.inner |= SecurityFlag::UNKNOWN0;
        self.unknown0 = Some(unknown0);
        self
    }

    pub const fn get_UNKNOWN0(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0> {
        self.unknown0.as_ref()
    }

    pub fn clear_UNKNOWN0(mut self) -> Self {
        self.inner &= SecurityFlag::UNKNOWN0.reverse_bits();
        self.unknown0 = None;
        self
    }

    pub const fn new_AUTHENTICATOR(authenticator: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            unknown0: None,
            authenticator: Some(authenticator),
        }
    }

    pub fn set_AUTHENTICATOR(mut self, authenticator: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator) -> Self {
        self.inner |= SecurityFlag::AUTHENTICATOR;
        self.authenticator = Some(authenticator);
        self
    }

    pub const fn get_AUTHENTICATOR(&self) -> Option<&CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator> {
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
impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag {
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
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
    pub pin_hash: [u8; 20],
    pub pin_salt: [u8; 16],
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
    pub(crate) fn size(&self) -> usize {
        20 * core::mem::size_of::<u8>() // pin_hash: u8[20]
        + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
    pub unknown0: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u64,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
    pub(crate) fn size(&self) -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
    pub unknown5: u8,
}

impl CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
    pub(crate) fn size(&self) -> usize {
        1 // unknown5: u8
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ClientOpcodeMessage;

    const RAW0: [u8; 135] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x02, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x01,
         0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xFE,
         0x00, 0xEE, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x01, 0x02, 0x03,
         0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
         0x10, 0x11, 0x12, 0x13, 0x14, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 344.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
                TelemetryKey {
                    unknown1: 0xFE,
                    unknown2: 0xDEADBEEE,
                    unknown3: [ 0x00, 0x01, 0x02, 0x03, ],
                    unknown4: [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, 0x14, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 344.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
                TelemetryKey {
                    unknown1: 0xFE,
                    unknown2: 0xDEADBEEE,
                    unknown3: [ 0x00, 0x01, 0x02, 0x03, ],
                    unknown4: [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, 0x14, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 344.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client0() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
                TelemetryKey {
                    unknown1: 0xFE,
                    unknown2: 0xDEADBEEE,
                    unknown3: [ 0x00, 0x01, 0x02, 0x03, ],
                    unknown4: [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                         0x13, 0x14, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 105] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x01, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x01,
         0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 401.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 401.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 401.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client1() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![
                TelemetryKey {
                    unknown1: 0xFF,
                    unknown2: 0xDEADBEEF,
                    unknown3: [ 0x01, 0x02, 0x03, 0x04, ],
                    unknown4: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                },
            ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 75] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 443.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 443.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 443.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client2() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 111] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01,
         0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
         0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 469.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 469.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 469.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client3() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    const RAW4: [u8; 123] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x03, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01,
         0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
         0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x10, 0x20, 0x30, 0x40, 0xEF, 0xBE,
         0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 503.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client4() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW4)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 503.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client4() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 503.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client4() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    const RAW5: [u8; 124] = [ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
         0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1, 0xF7,
         0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7, 0x08,
         0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE, 0x5C,
         0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D, 0xE1,
         0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A,
         0x58, 0xBB, 0x00, 0xD0, 0x00, 0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01,
         0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
         0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x10, 0x20, 0x30, 0x40, 0xEF, 0xBE,
         0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 547.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Client5() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                .set_AUTHENTICATOR(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    unknown5: 0x1,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW5)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 547.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Client5() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                .set_AUTHENTICATOR(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    unknown5: 0x1,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_client.wowm` line 547.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Client5() {
        let expected = CMD_AUTH_LOGON_PROOF_Client {
            client_public_key: [ 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8, 0xA9,
                 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
                 0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D,
                 0xD7, 0x08, ],
            client_proof: [ 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8,
                 0xF2, 0xDE, 0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, ],
            crc_hash: [ 0x4E, 0xF5, 0x2D, 0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC,
                 0xC8, 0x41, 0xEE, 0xB8, 0x90, 0x8A, 0x58, 0xBB, 0x00, 0xD0, ],
            telemetry_keys: vec![ ],
            security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::empty()
                .set_PIN(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin {
                    pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                         0x12, 0x13, ],
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                })
                .set_UNKNOWN0(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Unknown0 {
                    unknown0: 0x10,
                    unknown1: 0x20,
                    unknown2: 0x30,
                    unknown3: 0x40,
                    unknown4: 0xDEADBEEF,
                })
                .set_AUTHENTICATOR(CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Authenticator {
                    unknown5: 0x1,
                })
                ,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

}

