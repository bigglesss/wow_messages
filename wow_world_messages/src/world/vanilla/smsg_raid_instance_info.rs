use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::RaidInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L9):
/// ```text
/// smsg SMSG_RAID_INSTANCE_INFO = 0x02CC {
///     u32 amount_of_raid_infos;
///     RaidInfo[amount_of_raid_infos] raid_infos;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_INFO {
    pub raid_infos: Vec<RaidInfo>,
}

impl crate::Message for SMSG_RAID_INSTANCE_INFO {
    const OPCODE: u32 = 0x02cc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::read_u32_le(r)?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            let o = RaidInfo::read(r)?;
            raid_infos.push(o);
        }

        Ok(Self {
            raid_infos,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_RAID_INSTANCE_INFO {}

impl SMSG_RAID_INSTANCE_INFO {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_raid_infos: u32
        + self.raid_infos.len() * 12 // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

