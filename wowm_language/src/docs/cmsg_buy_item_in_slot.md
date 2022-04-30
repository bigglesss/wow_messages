## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
    Guid vendor_guid;
    u32 item_id;
    Guid bag_guid;
    u8 bag_slot;
    u8 amount;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | vendor_guid |  |
| 0x0E | 4 / Little | u32 | item_id |  |
| 0x12 | 8 / Little | [Guid](../spec/packed-guid.md) | bag_guid |  |
| 0x1A | 1 / - | u8 | bag_slot |  |
| 0x1B | 1 / - | u8 | amount |  |