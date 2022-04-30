## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SPELLORDAMAGE_IMMUNE = 0x0263 {
    Guid caster_guid;
    Guid target_guid;
    u32 id;
    u8 unknown1;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | caster_guid |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | target_guid |  |
| 0x14 | 4 / Little | u32 | id |  |
| 0x18 | 1 / - | u8 | unknown1 |  |