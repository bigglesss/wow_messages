use std::convert::{TryFrom, TryInto};
use crate::logon::version_3::{SecurityFlag, SecurityFlagError};
use crate::logon::version_2::TelemetryKey;
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Client {
    pub client_public_key: [u8; 32],
    pub client_proof: [u8; 20],
    pub crc_hash: [u8; 20],
    pub telemetry_keys: Vec<TelemetryKey>,
    pub security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag,
}

impl ClientMessage for CMD_AUTH_LOGON_PROOF_Client {
    const OPCODE: u8 = 0x01;
}
impl ReadableAndWritable for CMD_AUTH_LOGON_PROOF_Client {
    type Error = CMD_AUTH_LOGON_PROOF_ClientError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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
        let security_flag = SecurityFlag::read(r)?;

        let security_flag_if = match security_flag {
            SecurityFlag::NONE => CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE,
            SecurityFlag::PIN => {
                // pin_salt: u8[16]
                let mut pin_salt = [0_u8; 16];
                r.read_exact(&mut pin_salt)?;

                // pin_hash: u8[20]
                let mut pin_hash = [0_u8; 20];
                r.read_exact(&mut pin_hash)?;

                CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::PIN {
                    pin_salt,
                    pin_hash,
                }
            }
        };

        Ok(Self {
            client_public_key,
            client_proof,
            crc_hash,
            telemetry_keys,
            security_flag: security_flag_if,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
            i.write(w)?;
        }

        // security_flag: SecurityFlag
        self.security_flag.write(w)?;

        match &self.security_flag {
            CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE => {}
            CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::PIN {
                pin_salt,
                pin_hash,
            } => {
                // pin_salt: u8[16]
                for i in pin_salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // pin_hash: u8[20]
                for i in pin_hash.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
        }

        Ok(())
    }

}

impl VariableSized for CMD_AUTH_LOGON_PROOF_Client {
    fn size(&self) -> usize {
        32 * core::mem::size_of::<u8>() // client_public_key: u8[32]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + self.telemetry_keys.iter().fold(0, |acc, x| acc + TelemetryKey::size()) // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + self.security_flag.size() // security_flag: SecurityFlag and subfields
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_Client {
    fn maximum_possible_size() -> usize {
        32 * core::mem::size_of::<u8>() // client_public_key: u8[32]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // crc_hash: u8[20]
        + 1 // number_of_telemetry_keys: u8
        + 255 * TelemetryKey::maximum_possible_size() // telemetry_keys: TelemetryKey[number_of_telemetry_keys]
        + SecurityFlag::maximum_possible_size() // security_flag: SecurityFlag
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_LOGON_PROOF_ClientError {
    Io(std::io::Error),
    SecurityFlag(SecurityFlagError),
}

impl std::error::Error for CMD_AUTH_LOGON_PROOF_ClientError {}
impl std::fmt::Display for CMD_AUTH_LOGON_PROOF_ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SecurityFlag(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_LOGON_PROOF_ClientError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SecurityFlagError> for CMD_AUTH_LOGON_PROOF_ClientError {
    fn from(e: SecurityFlagError) -> Self {
        Self::SecurityFlag(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    NONE,
    PIN {
        pin_salt: [u8; 16],
        pin_hash: [u8; 20],
    },
}

impl From<&SecurityFlag> for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn from(e: &SecurityFlag) -> Self {
        match &e {
            SecurityFlag::NONE => Self::NONE,
            SecurityFlag::PIN => Self::PIN {
                pin_salt: Default::default(),
                pin_hash: Default::default(),
            },
        }
    }
}

impl From<&CMD_AUTH_LOGON_PROOF_ClientSecurityFlag> for SecurityFlag {
    fn from(v: &CMD_AUTH_LOGON_PROOF_ClientSecurityFlag) -> Self {
        match &v {
            CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE => Self::NONE,
            CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::PIN { .. } => Self::PIN,
        }
    }
}

impl Default for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn size(&self) -> usize {
        match self {
            Self::NONE =>  {
                1
            }
            Self::PIN  {
                pin_salt,
                pin_hash,
            } => {
                1
                + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
                + 20 * core::mem::size_of::<u8>() // pin_hash: u8[20]
            }
        }
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ClientSecurityFlag {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_LOGON_PROOF_Client;
    use crate::VariableSized;
    use crate::logon::version_3::SecurityFlag;
    use crate::logon::version_2::TelemetryKey;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x02, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
             0xFE, 0x00, 0xEE, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x01, 0x02,
             0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
             0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x00, ];

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
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client1() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x01, 0xFF, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13,
             0x00, ];

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
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client2() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x00, ];

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
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::NONE,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Client3() {
        let raw: Vec<u8> = vec![ 0x01, 0xF1, 0x3E, 0xE5, 0xD1, 0x83, 0xC4, 0xC8,
             0xA9, 0x50, 0x0E, 0x3F, 0x5A, 0x5D, 0x8A, 0xEE, 0x4E, 0x2E, 0x45, 0xE1,
             0xF7, 0xCC, 0x8F, 0x1C, 0xF5, 0xEE, 0x8E, 0x11, 0xCE, 0xD3, 0x1D, 0xD7,
             0x08, 0x6B, 0x1E, 0x48, 0x1B, 0x4D, 0x04, 0xA1, 0x18, 0xD8, 0xF2, 0xDE,
             0x5C, 0x59, 0xD5, 0x5C, 0x81, 0x2E, 0x65, 0xEC, 0x3E, 0x4E, 0xF5, 0x2D,
             0xE1, 0x80, 0x5E, 0x1A, 0x67, 0x15, 0xEC, 0xC8, 0x41, 0xEE, 0xB8, 0x90,
             0x8A, 0x58, 0xBB, 0x00, 0xD0, 0x00, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00,
             0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
             0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ];

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
            security_flag: CMD_AUTH_LOGON_PROOF_ClientSecurityFlag::PIN {
                pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                     0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                pin_hash: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                     0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
                     0x13, ],
            },
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.client_public_key, expected.client_public_key);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.crc_hash, expected.crc_hash);
        assert_eq!(t.telemetry_keys, expected.telemetry_keys);
        assert_eq!(t.security_flag, expected.security_flag);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
