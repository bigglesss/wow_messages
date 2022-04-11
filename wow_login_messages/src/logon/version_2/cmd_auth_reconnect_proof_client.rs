use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm#L3):
/// ```text
/// clogin CMD_AUTH_RECONNECT_PROOF_Client = 0x3 {
///     u8[16] proof_data;
///     u8[20] client_proof;
///     u8[20] client_checksum;
///     u8 key_count = 0;
/// }
/// ```
pub struct CMD_AUTH_RECONNECT_PROOF_Client {
    pub proof_data: [u8; 16],
    pub client_proof: [u8; 20],
    pub client_checksum: [u8; 20],
}

impl ClientMessage for CMD_AUTH_RECONNECT_PROOF_Client {
    const OPCODE: u8 = 0x03;
}
impl CMD_AUTH_RECONNECT_PROOF_Client {
    /// The field `key_count` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const KEY_COUNT_VALUE: u8 = 0x00;

}

impl ReadableAndWritable for CMD_AUTH_RECONNECT_PROOF_Client {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // proof_data: u8[16]
        let mut proof_data = [0_u8; 16];
        r.read_exact(&mut proof_data)?;

        // client_proof: u8[20]
        let mut client_proof = [0_u8; 20];
        r.read_exact(&mut client_proof)?;

        // client_checksum: u8[20]
        let mut client_checksum = [0_u8; 20];
        r.read_exact(&mut client_checksum)?;

        // key_count: u8
        let _key_count = crate::util::read_u8_le(r)?;
        // key_count is expected to always be 0 (0)

        Ok(Self {
            proof_data,
            client_proof,
            client_checksum,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // proof_data: u8[16]
        for i in self.proof_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // client_checksum: u8[20]
        for i in self.client_checksum.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // key_count: u8
        w.write_all(&Self::KEY_COUNT_VALUE.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for CMD_AUTH_RECONNECT_PROOF_Client {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMD_AUTH_RECONNECT_PROOF_Client {
    fn maximum_possible_size() -> usize {
        16 * core::mem::size_of::<u8>() // proof_data: u8[16]
        + 20 * core::mem::size_of::<u8>() // client_proof: u8[20]
        + 20 * core::mem::size_of::<u8>() // client_checksum: u8[20]
        + 1 // key_count: u8
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_RECONNECT_PROOF_Client;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_client.wowm` line 10.
    #[test]
    fn CMD_AUTH_RECONNECT_PROOF_Client0() {
        let raw: Vec<u8> = vec![ 0x03, 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B,
             0xF2, 0xF9, 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, 0x6B, 0x6D, 0x1A,
             0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38, 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05,
             0x71, 0xBA, 0x47, 0x8C, 0xA3, 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6,
             0x82, 0xED, 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0,
             0xF5, 0x00, ];

        let expected = CMD_AUTH_RECONNECT_PROOF_Client {
            proof_data: [ 0xEA, 0xFA, 0xB9, 0xC6, 0x18, 0x15, 0x0B, 0xF2, 0xF9,
                 0x32, 0xCE, 0x27, 0x62, 0x79, 0x96, 0x99, ],
            client_proof: [ 0x6B, 0x6D, 0x1A, 0x0D, 0xF3, 0xA5, 0x9E, 0x6A, 0x38,
                 0x02, 0xE7, 0x0B, 0xE1, 0x2F, 0x05, 0x71, 0xBA, 0x47, 0x8C, 0xA3, ],
            client_checksum: [ 0x28, 0xA7, 0x9E, 0x9A, 0x24, 0x28, 0xE6, 0x82, 0xED,
                 0xEC, 0xC7, 0xC9, 0xE8, 0x6E, 0xF1, 0x3B, 0x7B, 0xE1, 0xE0, 0xF5, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.proof_data, expected.proof_data);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.client_checksum, expected.client_checksum);

        assert_eq!(CMD_AUTH_RECONNECT_PROOF_Client::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
