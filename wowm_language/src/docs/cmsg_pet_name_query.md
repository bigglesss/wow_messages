## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_PET_NAME_QUERY = 0x0052 {
    u32 pet_number;
    Guid guid;
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
| 0x06 | 4 / Little | u32 | pet_number |  |
| 0x0A | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
### Examples
```c
0, 16, // size
82, 0, 0, 0, // opcode (82)
239, 190, 173, 222, // pet_number: u32
239, 190, 173, 222, 222, 202, 250, 0, // guid: Guid
```