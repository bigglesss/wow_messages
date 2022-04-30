## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_DUEL_REQUESTED = 0x0167 {
    Guid initiator_guid;
    Guid target_guid;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | initiator_guid |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | target_guid |  |