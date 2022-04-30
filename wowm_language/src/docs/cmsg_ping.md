## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_PING = 0x01DC {
    u32 sequence_id;
    u32 round_time_in_ms;
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
| 0x06 | 4 / Little | u32 | sequence_id |  |
| 0x0A | 4 / Little | u32 | round_time_in_ms |  |
### Examples
```c
0, 12, // size
220, 1, 0, 0, // opcode (476)
239, 190, 173, 222, // sequence_id: u32
222, 202, 250, 0, // round_time_in_ms: u32
```