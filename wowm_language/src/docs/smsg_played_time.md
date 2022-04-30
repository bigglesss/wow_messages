## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PLAYED_TIME = 0x01CD {
    u32 total_played_time;
    u32 level_played_time;
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
| 0x04 | 4 / Little | u32 | total_played_time |  |
| 0x08 | 4 / Little | u32 | level_played_time |  |