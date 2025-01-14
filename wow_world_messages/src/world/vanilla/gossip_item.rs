use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L3):
/// ```text
/// struct GossipItem {
///     u32 id;
///     u8 item_icon;
///     Bool coded;
/// }
/// ```
pub struct GossipItem {
    /// vmangos: sets to loop index
    ///
    pub id: u32,
    pub item_icon: u8,
    /// vmangos: makes pop up box password
    ///
    pub coded: bool,
}

impl GossipItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: Bool
        w.write_all(if self.coded { &[1] } else { &[0] })?;

        Ok(())
    }
}

impl GossipItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: Bool
        let coded = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

}

