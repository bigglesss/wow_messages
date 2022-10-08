use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::channel_member_flags123::ChannelMemberFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L24):
/// ```text
/// struct ChannelMember {
///     Guid guid;
///     ChannelMemberFlags member_flags;
/// }
/// ```
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: ChannelMemberFlags,
}

impl ChannelMember {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // member_flags: ChannelMemberFlags
        w.write_all(&(self.member_flags.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
}

impl ChannelMember {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // member_flags: ChannelMemberFlags
        let member_flags = ChannelMemberFlags::new(crate::util::read_u8_le(r)?);

        Ok(Self {
            guid,
            member_flags,
        })
    }

}

