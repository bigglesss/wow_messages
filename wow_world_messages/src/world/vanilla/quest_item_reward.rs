use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L51):
/// ```text
/// struct QuestItemReward {
///     u32 item;
///     u32 item_count;
/// }
/// ```
pub struct QuestItemReward {
    pub item: u32,
    pub item_count: u32,
}

impl QuestItemReward {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        Ok(())
    }
}

impl QuestItemReward {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
        })
    }

}

