## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_BUY_ITEM = 0x01A4 {
    Guid guid;
    u32 vendor_slot;
    u32 amount_for_sale;
    u32 amount_bought;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0C | 4 / Little | u32 | vendor_slot |  |
| 0x10 | 4 / Little | u32 | amount_for_sale |  |
| 0x14 | 4 / Little | u32 | amount_bought |  |