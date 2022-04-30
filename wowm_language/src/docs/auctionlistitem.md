## Client Version 1.12

### Wowm Representation
```rust,ignore
struct AuctionListItem {
    u32 id;
    u32 item_entry;
    u32 item_enchantment;
    u32 item_random_property_id;
    u32 item_suffix_factor;
    u32 item_count;
    u32 item_charges;
    Guid item_owner;
    u32 start_bid;
    u32 minimum_bid;
    u32 buyout_amount;
    u32 time_left_in_msecs;
    Guid highest_bidder;
    u32 highest_bid;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | id |  |
| 0x04 | 4 / Little | u32 | item_entry |  |
| 0x08 | 4 / Little | u32 | item_enchantment |  |
| 0x0C | 4 / Little | u32 | item_random_property_id |  |
| 0x10 | 4 / Little | u32 | item_suffix_factor |  |
| 0x14 | 4 / Little | u32 | item_count |  |
| 0x18 | 4 / Little | u32 | item_charges |  |
| 0x1C | 8 / Little | [Guid](../spec/packed-guid.md) | item_owner |  |
| 0x24 | 4 / Little | u32 | start_bid |  |
| 0x28 | 4 / Little | u32 | minimum_bid |  |
| 0x2C | 4 / Little | u32 | buyout_amount |  |
| 0x30 | 4 / Little | u32 | time_left_in_msecs |  |
| 0x34 | 8 / Little | [Guid](../spec/packed-guid.md) | highest_bidder |  |
| 0x3C | 4 / Little | u32 | highest_bid |  |