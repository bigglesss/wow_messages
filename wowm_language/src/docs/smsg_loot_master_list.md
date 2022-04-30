## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOOT_MASTER_LIST = 0x02A4 {
    u8 amount_of_players;
    Guid[amount_of_players] guids;
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
| 0x04 | 1 / - | u8 | amount_of_players |  |
| 0x05 | ? / - | [Guid](../spec/packed-guid.md)[amount_of_players] | guids |  |