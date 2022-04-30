## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_CAST_FAILED = 0x0138 {
    u32 id;
    u8 unknown1;
    SpellCastResult result;
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
| 0x04 | 4 / Little | u32 | id |  |
| 0x08 | 1 / - | u8 | unknown1 |  |
| 0x09 | ? / - | [SpellCastResult](spellcastresult.md) | result |  |