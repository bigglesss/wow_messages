use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_owner_items.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_owner_items.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_LIST_OWNER_ITEMS = 0x0259 {
///     Guid auctioneer_guid;
///     u32 list_from;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_from: u32,
}

impl crate::Message for CMSG_AUCTION_LIST_OWNER_ITEMS {
    const OPCODE: u32 = 0x0259;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // list_from: u32
        let list_from = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_from,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_AUCTION_LIST_OWNER_ITEMS {}

