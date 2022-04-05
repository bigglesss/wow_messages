use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L9):
/// ```text
/// slogin CMD_XFER_DATA = 0x31 {
///     u16 size;
///     u8[size] data;
/// }
/// ```
pub struct CMD_XFER_DATA {
    pub data: Vec<u8>,
}

impl ServerMessage for CMD_XFER_DATA {
    const OPCODE: u8 = 0x31;
}
impl ReadableAndWritable for CMD_XFER_DATA {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // size: u16
        let size = crate::util::read_u16_le(r)?;

        // data: u8[size]
        let mut data = Vec::with_capacity(size as usize);
        for i in 0..size {
            data.push(crate::util::read_u8_le(r)?);
        }

        Ok(Self {
            data,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // size: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes())?;

        // data: u8[size]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

}

impl VariableSized for CMD_XFER_DATA {
    fn size(&self) -> usize {
        2 // size: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[size]
    }
}

impl MaximumPossibleSized for CMD_XFER_DATA {
    fn maximum_possible_size() -> usize {
        2 // size: u16
        + 65535 * core::mem::size_of::<u8>() // data: u8[size]
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_XFER_DATA;
    use crate::VariableSized;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 14.
    #[test]
    fn CMD_XFER_DATA0() {
        let raw: Vec<u8> = vec![ 0x31, 0x01, 0x00, 0xFF, ];

        let expected = CMD_XFER_DATA {
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 23.
    #[test]
    fn CMD_XFER_DATA1() {
        let raw: Vec<u8> = vec![ 0x31, 0x00, 0x00, ];

        let expected = CMD_XFER_DATA {
            data: vec![ ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
