## Client Version 1.12

### Comment

cmangos/vmangos/mangoszero: All fields with 'skip' are completely unused

### Wowm Representation
```rust,ignore
cmsg CMSG_PETITION_BUY = 0x01BD {
    Guid npc;
    u32 skip1;
    Guid skip2;
    CString name;
    u32 skip3;
    u32 skip4;
    u32 skip5;
    u32 skip6;
    u32 skip7;
    u32 skip8;
    u32 skip9;
    u32 skip10;
    u32 skip11;
    u32 skip12;
    u16 skip13;
    u8 skip14;
    u32 index;
    u32 skip15;
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
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | npc |  |
| 0x0E | 4 / Little | u32 | skip1 |  |
| 0x12 | 8 / Little | [Guid](../spec/packed-guid.md) | skip2 |  |
| 0x1A | - / - | CString | name |  |
| - | 4 / Little | u32 | skip3 |  |
| - | 4 / Little | u32 | skip4 |  |
| - | 4 / Little | u32 | skip5 |  |
| - | 4 / Little | u32 | skip6 |  |
| - | 4 / Little | u32 | skip7 |  |
| - | 4 / Little | u32 | skip8 |  |
| - | 4 / Little | u32 | skip9 |  |
| - | 4 / Little | u32 | skip10 |  |
| - | 4 / Little | u32 | skip11 |  |
| - | 4 / Little | u32 | skip12 |  |
| - | 2 / Little | u16 | skip13 |  |
| - | 1 / - | u8 | skip14 |  |
| - | 4 / Little | u32 | index |  |
| - | 4 / Little | u32 | skip15 |  |