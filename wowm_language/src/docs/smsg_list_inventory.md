## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LIST_INVENTORY = 0x019F {
    Guid vendor;
    u8 amount_of_items;
    ListInventoryItem[amount_of_items] items;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | vendor |  |
| 0x0C | 1 / - | u8 | amount_of_items |  |
| 0x0D | ? / - | [ListInventoryItem](listinventoryitem.md)[amount_of_items] | items |  |