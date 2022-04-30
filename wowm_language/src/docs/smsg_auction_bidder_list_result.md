## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x0265 {
    u32 count;
    AuctionListItem[count] auctions;
    u32 total_amount_of_auctions;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 4 / Little | u32 | count |  |
| 0x08 | ? / - | [AuctionListItem](auctionlistitem.md)[count] | auctions |  |
| - | 4 / Little | u32 | total_amount_of_auctions |  |