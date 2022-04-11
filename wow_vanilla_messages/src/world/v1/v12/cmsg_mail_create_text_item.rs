use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/cmsg_mail_create_text_item.wowm#L3):
/// ```text
/// cmsg CMSG_MAIL_CREATE_TEXT_ITEM = 0x24A {
///     Guid mailbox_guid;
///     u32 mail_id;
///     u32 mail_template_id;
/// }
/// ```
pub struct CMSG_MAIL_CREATE_TEXT_ITEM {
    pub mailbox_guid: Guid,
    pub mail_id: u32,
    pub mail_template_id: u32,
}

impl WorldClientMessageWrite for CMSG_MAIL_CREATE_TEXT_ITEM {
    const OPCODE: u32 = 0x24a;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_MAIL_CREATE_TEXT_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox_guid: Guid
        let mailbox_guid = Guid::read(r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(r)?;

        // mail_template_id: u32
        let mail_template_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox_guid,
            mail_id,
            mail_template_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox_guid: Guid
        self.mailbox_guid.write(w)?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_MAIL_CREATE_TEXT_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_MAIL_CREATE_TEXT_ITEM {
    fn maximum_possible_size() -> usize {
        8 // mailbox_guid: Guid
        + 4 // mail_id: u32
        + 4 // mail_template_id: u32
    }
}

