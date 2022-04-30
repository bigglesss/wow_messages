use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_SEND_MAIL {
    pub mailbox: Guid,
    pub receiver: String,
    pub subject: String,
    pub body: String,
    pub unknown1: u32,
    pub unknown2: u32,
    pub item: Guid,
    pub money: u32,
    pub cash_on_delivery_amount: u32,
    pub unknown3: u32,
    pub unknown4: u32,
}

impl ClientMessageWrite for CMSG_SEND_MAIL {
    const OPCODE: u32 = 0x238;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_SEND_MAIL {
    type Error = CMSG_SEND_MAILError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        // receiver: CString
        let receiver = crate::util::read_c_string_to_vec(r)?;
        let receiver = String::from_utf8(receiver)?;

        // subject: CString
        let subject = crate::util::read_c_string_to_vec(r)?;
        let subject = String::from_utf8(subject)?;

        // body: CString
        let body = crate::util::read_c_string_to_vec(r)?;
        let body = String::from_utf8(body)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox,
            receiver,
            subject,
            body,
            unknown1,
            unknown2,
            item,
            money,
            cash_on_delivery_amount,
            unknown3,
            unknown4,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox: Guid
        self.mailbox.write(w)?;

        // receiver: CString
        w.write_all(self.receiver.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // subject: CString
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body: CString
        w.write_all(self.body.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // item: Guid
        self.item.write(w)?;

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_SEND_MAIL {
    fn size(&self) -> usize {
        8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString and Null Terminator
        + self.subject.len() + 1 // subject: CString and Null Terminator
        + self.body.len() + 1 // body: CString and Null Terminator
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 8 // item: Guid
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

impl MaximumPossibleSized for CMSG_SEND_MAIL {
    fn maximum_possible_size() -> usize {
        8 // mailbox: Guid
        + 256 // receiver: CString
        + 256 // subject: CString
        + 256 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 8 // item: Guid
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

#[derive(Debug)]
pub enum CMSG_SEND_MAILError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_SEND_MAILError {}
impl std::fmt::Display for CMSG_SEND_MAILError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SEND_MAILError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_SEND_MAILError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

