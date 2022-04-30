## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SPELL_UPDATE_CHAIN_TARGETS = 0x0330 {
    Guid caster;
    u32 spell;
    u32 amount_of_targets;
    Guid[amount_of_targets] targets;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | caster |  |
| 0x0C | 4 / Little | u32 | spell |  |
| 0x10 | 4 / Little | u32 | amount_of_targets |  |
| 0x14 | ? / - | [Guid](../spec/packed-guid.md)[amount_of_targets] | targets |  |