use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// mangoszero/vmangos: No idea when this is called.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L3):
/// ```text
/// smsg MSG_QUERY_NEXT_MAIL_TIME_Server = 0x0284 {
///     f32 unread_mails;
/// }
/// ```
pub struct MSG_QUERY_NEXT_MAIL_TIME_Server {
    /// mangoszero sets 0 if has unread mail, -86400.0f (0xC7A8C000) if not
    /// vmangos sets 0 if has unread mail, -1.0f if not
    /// cmangos has the behavior of mangoszero except when there are unread mails. This is TODO.
    ///
    pub unread_mails: f32,
}

impl crate::Message for MSG_QUERY_NEXT_MAIL_TIME_Server {
    const OPCODE: u32 = 0x0284;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unread_mails: f32
        w.write_all(&self.unread_mails.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // unread_mails: f32
        let unread_mails = crate::util::read_f32_le(r)?;
        Ok(Self {
            unread_mails,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_QUERY_NEXT_MAIL_TIME_Server {}

